use {
    std::{
        borrow::Cow,
        cmp::Ordering::{
            self,
            *,
        },
        collections::{
            HashMap,
            HashSet,
        },
    },
    enum_iterator::all,
    itertools::Itertools as _,
    omsim_rs::data::*,
    rocket::{
        http::Status,
        response::content::{
            RawCss,
            RawHtml,
            RawJavaScript,
        },
        serde::json::Json,
        uri,
    },
    rocket_util::{
        Doctype,
        html,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    crate::util::IteratorExt as _,
};

mod molecules;
mod util;

#[derive(Serialize)]
enum InOut {
    Reagent,
    Product,
    Both,
}

trait MoleculeExt {
    fn position_normalized(&self) -> Self;
    fn normalized(&self) -> Self;
    fn mirrored(&self) -> Self;
    fn draw(&self, id: &str) -> RawHtml<String>;
}

impl MoleculeExt for Molecule {
    fn position_normalized(&self) -> Self {
        let offset = HexIndex {
            q: self.atoms.keys().map(|&HexIndex { q, .. }| q).min().unwrap_or_default(),
            r: self.atoms.keys().map(|&HexIndex { r, .. }| r).min().unwrap_or_default(),
        };
        let mut normalized = self.mapped_positions(|pos| pos - offset);
        normalized.bonds = normalized.bonds.into_iter().map(|Bond { start, end, ty }| Bond {
            start: if (start.q, start.r) <= (end.q, end.r) { start } else { end },
            end: if (start.q, start.r) <= (end.q, end.r) { end } else { start },
            ty,
        }).collect();
        normalized
    }

    fn normalized(&self) -> Self {
        fn atom_id(atom: Atom) -> u8 {
            match atom {
                Atom::Salt => 1,
                Atom::Air => 2,
                Atom::Earth => 3,
                Atom::Fire => 4,
                Atom::Water => 5,
                Atom::Quicksilver => 6,
                Atom::Gold => 7,
                Atom::Silver => 8,
                Atom::Copper => 9,
                Atom::Iron => 10,
                Atom::Tin => 11,
                Atom::Lead => 12,
                Atom::Vitae => 13,
                Atom::Mors => 14,
                Atom::Repeat => 15,
                Atom::Quintessence => 16,
            }
        }

        fn cmp_atoms((k1, v1): (&HexIndex, &Atom), (k2, v2): (&HexIndex, &Atom)) -> Ordering {
            k1.q.cmp(&k2.q)
            .then_with(|| k1.r.cmp(&k2.r))
            .then_with(|| atom_id(*v1).cmp(&atom_id(*v2)))
        }

        fn cmp_atoms_ref(&p1: &(&HexIndex, &Atom), &p2: &(&HexIndex, &Atom)) -> Ordering {
            cmp_atoms(p1, p2)
        }

        fn cmp_bond_types(t1: &BondType, t2: &BondType) -> Ordering {
            match (t1, t2) {
                (BondType::Normal, BondType::Normal) => Equal,
                (BondType::Normal, BondType::Triplex { .. }) => Less,
                (BondType::Triplex { .. }, BondType::Normal) => Greater,
                (BondType::Triplex { red: r1, black: b1, yellow: y1 }, BondType::Triplex { red: r2, black: b2, yellow: y2 }) => r1.cmp(r2).then_with(|| b1.cmp(b2)).then_with(|| y1.cmp(y2)),
            }
        }

        fn cmp_bonds(b1: &Bond, b2: &Bond) -> Ordering {
            let b1_start = (b1.start.q, b1.start.r);
            let b1_end = (b1.end.q, b1.end.r);
            let b1_min = b1_start.min(b1_end);
            let b1_max = b1_start.max(b1_end);
            let b2_start = (b2.start.q, b2.start.r);
            let b2_end = (b2.end.q, b2.end.r);
            let b2_min = b2_start.min(b2_end);
            let b2_max = b2_start.max(b2_end);
            b1_min.cmp(&b2_min)
            .then_with(|| b1_max.cmp(&b2_max))
            .then_with(|| cmp_bond_types(&b1.ty, &b2.ty))
        }

        fn cmp_bonds_ref(&b1: &&Bond, &b2: &&Bond) -> Ordering {
            cmp_bonds(b1, b2)
        }

        if self.atoms.values().any(|&atom| atom == Atom::Repeat) {
            self.position_normalized()
        } else {
            all().map(|rotation| self.rotated(HexIndex::default(), rotation).position_normalized()).min_by(|m1, m2|
                m1.atoms.iter().sorted_unstable_by(cmp_atoms_ref)._cmp_by(m2.atoms.iter().sorted_unstable_by(cmp_atoms_ref), cmp_atoms)
                .then_with(|| m1.bonds.iter().sorted_unstable_by(cmp_bonds_ref)._cmp_by(m2.bonds.iter().sorted_unstable_by(cmp_bonds_ref), cmp_bonds))
            ).expect("all::<Rotation>() is nonempty") //TODO make a nonempty variant of all()
        }
    }

    fn mirrored(&self) -> Self {
        self.mapped_positions(|pos| HexIndex { q: -pos.s(), r: -pos.r, }).position_normalized()
    }

    fn draw(&self, id: &str) -> RawHtml<String> {
        let Self { atoms, bonds } = self.mirrored();
        let min_x = atoms.keys().map(|&HexIndex { q, r }| 2 * q + r).min().unwrap_or_default();
        let width = atoms.keys().map(|&HexIndex { q, r }| 2 * q + r + 2).max().unwrap_or_default() - min_x;
        let height = atoms.keys().map(|&HexIndex { r, .. }| r + 1).max().unwrap_or_default();
        let width = (41 * width + 10) * 3 / 4;
        let height = (71 * height + 20) * 3 / 4;
        html! {
            canvas(id = id);
            script {
                : RawHtml(format!("
                    const productCanvas{id} = document.getElementById({id:?});
                    productCanvas{id}.width = {width} * window.devicePixelRatio;
                    productCanvas{id}.style.width = '{width}px';
                    productCanvas{id}.height = {height} * window.devicePixelRatio;
                    productCanvas{id}.style.height = '{height}px';
                    const pctx{id} = productCanvas{id}.getContext('2d');
                    pctx{id}.scale(window.devicePixelRatio, window.devicePixelRatio);
                    pctx{id}.scale(0.75, 0.75);
                    pctx{id}.fillStyle = '#223';
                    pctx{id}.fillRect(0, 0, productCanvas{id}.width, productCanvas{id}.height);
                    for (let shadow = 4; shadow >= 0; shadow -= 4) {{
                "));
                @for Bond { start, end, ty } in bonds {
                    @let _ = ty; //TODO
                    : RawHtml(format!("drawProductBond(pctx{id}, {}, {min_x}, {}, {}, {}/6, shadow);\n", match ty {
                        BondType::Normal => Cow::Borrowed("false, false, false"),
                        BondType::Triplex { red, black, yellow } => Cow::Owned(format!("{red}, {black}, {yellow}")),
                    }, start.q, start.r, match end - start {
                        HexIndex { q: 1, r: 0 } => 0,
                        HexIndex { q: 0, r: 1 } => 1,
                        HexIndex { q: -1, r: 1 } => 2,
                        HexIndex { q: -1, r: 0 } => 3,
                        HexIndex { q: 0, r: -1 } => 4,
                        HexIndex { q: 1, r: -1 } => 5,
                        _ => unimplemented!("quantum bond"),
                    }));
                }
                : RawHtml("if (!shadow) {\n");
                @for (coords, atom) in &atoms {
                    : RawHtml(format!("drawProductAtom(pctx{id}, '{}', {min_x}, {}, {}, 2);\n", format_atom(*atom), coords.q, coords.r));
                }
                : RawHtml("}\n");
                @for (coords, atom) in atoms {
                    : RawHtml(format!("drawProductAtom(pctx{id}, '{}', {min_x}, {}, {}, shadow);\n", format_atom(atom), coords.q, coords.r));
                }
                : RawHtml("}\n");
            }
        }
    }
}

fn parse_atom(s: &str) -> Option<Atom> {
    match s {
        "Salt" | "salt" => Some(Atom::Salt),
        "Air" | "air" => Some(Atom::Air),
        "Earth" | "earth" => Some(Atom::Earth),
        "Fire" | "fire" => Some(Atom::Fire),
        "Water" | "water" => Some(Atom::Water),
        "Quicksilver" | "quicksilver" => Some(Atom::Quicksilver),
        "Gold" | "gold" => Some(Atom::Gold),
        "Silver" | "silver" => Some(Atom::Silver),
        "Copper" | "copper" => Some(Atom::Copper),
        "Iron" | "iron" => Some(Atom::Iron),
        "Tin" | "tin" => Some(Atom::Tin),
        "Lead" | "lead" => Some(Atom::Lead),
        "Vitae" | "vitae" => Some(Atom::Vitae),
        "Mors" | "mors" => Some(Atom::Mors),
        "Repeat" | "repeat" => Some(Atom::Repeat),
        "Quintessence" | "quintessence" => Some(Atom::Quintessence),
        _ => None,
    }
}

fn format_atom(atom: Atom) -> &'static str {
    match atom {
        Atom::Salt => "salt",
        Atom::Air => "air",
        Atom::Earth => "earth",
        Atom::Fire => "fire",
        Atom::Water => "water",
        Atom::Quicksilver => "quicksilver",
        Atom::Gold => "gold",
        Atom::Silver => "silver",
        Atom::Copper => "copper",
        Atom::Iron => "iron",
        Atom::Tin => "tin",
        Atom::Lead => "lead",
        Atom::Vitae => "vitae",
        Atom::Mors => "mors",
        Atom::Repeat => "repeat",
        Atom::Quintessence => "quintessence",
    }
}

#[rocket::get("/")]
fn index() -> RawHtml<String> {
    html! {
        : Doctype;
        html {
            head {
                meta(charset = "utf-8");
                title : "Opus Magnum Molecule Database";
                meta(name = "viewport", content = "width=device-width, initial-scale=1, shrink-to-fit=no");
                link(rel = "stylesheet", href = "/static/common.css");
                script(src = "/static/common.js");
                script(defer, src = "/static/transmogrification.js");
            }
            body {
                main(style = "flex-direction: column;") {
                    div {
                        h2 : "ENTER MOLECULE TO LOOK UP";
                        canvas(id = "current");
                        p(id = "result", style = "display: none;");
                        p(id = "error");
                    }
                    ul(id = "default") {
                        li {
                            a(href = uri!(molecules_list).to_string()) : "List of all molecules";
                        }
                    }
                }
                canvas(id = "next", style = "display: none;");
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsState {
    #[allow(unused)] selected_atom: Option<String>,
    #[allow(unused)] selected_bond: Option<String>,
    #[serde(flatten)]
    rest: HashMap<String, String>,
}

#[derive(Serialize)]
struct MoleculeResponse {
    appearances: Vec<(String, InOut, String)>,
}

#[rocket::post("/api/v1/molecule-from-state", format = "json", data = "<state>")]
fn molecule_from_state(state: Json<JsState>) -> Result<Json<MoleculeResponse>, Status> {
    let Json(JsState { rest, .. }) = state;
    let mut molecule = Molecule { atoms: HashMap::default(), bonds: HashSet::default() };
    for (key, value) in rest {
        if let Some((start, end)) = key.split_once(':') {
            let (q1, r1) = start.split_once(',').ok_or(Status::BadRequest)?;
            let (q2, r2) = end.split_once(',').ok_or(Status::BadRequest)?;
            molecule.bonds.insert(Bond {
                start: HexIndex { q: q1.parse().map_err(|_| Status::BadRequest)?, r: r1.parse().map_err(|_| Status::BadRequest)? },
                end: HexIndex { q: q2.parse().map_err(|_| Status::BadRequest)?, r: r2.parse().map_err(|_| Status::BadRequest)? },
                ty: if value == "n" {
                    BondType::Normal
                } else {
                    BondType::Triplex { red: value.contains('r'), black: value.contains('k'), yellow: value.contains('y') }
                },
            });
        } else {
            let (q, r) = key.split_once(',').ok_or(Status::BadRequest)?;
            molecule.atoms.insert(HexIndex { q: q.parse().map_err(|_| Status::BadRequest)?, r: r.parse().map_err(|_| Status::BadRequest)? }, parse_atom(&value).ok_or(Status::BadRequest)?);
        }
    }
    let molecule = molecule.normalized();
    for (iter_molecule, appearances) in molecules::molecules() {
        if iter_molecule == molecule {
            return Ok(Json(MoleculeResponse { appearances: appearances.into_iter().map(|(puzzle_name, inout, name)| (puzzle_name.to_owned(), inout, name.to_owned())).collect() }))
        }
    }
    Ok(Json(MoleculeResponse { appearances: Vec::default() }))
}

#[rocket::get("/molecules")]
fn molecules_list() -> RawHtml<String> {
    html! {
        : Doctype;
        html {
            head {
                meta(charset = "utf-8");
                title : "Opus Magnum Molecule Database";
                meta(name = "viewport", content = "width=device-width, initial-scale=1, shrink-to-fit=no");
                link(rel = "stylesheet", href = "/static/common.css");
                script(src = "/static/common.js");
            }
            body {
                main {
                    @for (idx, (molecule, appearances)) in molecules::molecules().into_iter().sorted_unstable_by_key(|(_, appearances)| appearances.iter().map(|(_, _, name)| name).min().map(|name| name.to_owned())).enumerate() {
                        div {
                            h2 : appearances.iter().map(|(_, _, name)| name).sorted_unstable().dedup().join("/");
                            : molecule.draw(&format!("product{idx}"));
                        }
                    }
                }
            }
        }
    }
}

#[rocket::get("/static/common.css")]
fn common_css() -> RawCss<&'static str> {
    RawCss(include_str!("../assets/static/common.css"))
}

#[rocket::get("/static/common.js")]
fn common_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../assets/static/common.js"))
}

#[rocket::get("/static/transmogrification.js")]
fn transmogrification_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../assets/static/transmogrification.js"))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::custom(rocket::Config {
        port: 42821,
        ..rocket::Config::default()
    }).mount("/", rocket::routes![
        index,
        molecule_from_state,
        molecules_list,
        common_css,
        common_js,
        transmogrification_js,
    ])
}
