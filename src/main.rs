use {
    std::{
        borrow::Cow,
        cmp::Ordering::{
            self,
            *,
        },
    },
    enum_iterator::all,
    itertools::Itertools as _,
    omsim_rs::data::*,
    rocket::response::content::{
        RawCss,
        RawHtml,
        RawJavaScript,
    },
    rocket_util::{
        Doctype,
        html,
    },
    crate::util::IteratorExt as _,
};

mod molecules;
mod util;

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
        self.mapped_positions(|pos| pos - offset)
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
            self.translated(HexIndex {
                q: -self.atoms.keys().map(|&HexIndex { q, .. }| q).min().unwrap_or_default(),
                r: -self.atoms.keys().map(|&HexIndex { r, .. }| r).min().unwrap_or_default(),
            })
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
                main {
                    div {
                        h2 : "CHOOSE YOUR REAGENT (UP TO THREE ATOMS)";
                        canvas(id = "current");
                        p(style = "display: none;") {
                            : "download: ";
                            a(id = "download", href = "#");
                            : " (";
                            a(href = "http://events.critelli.technology/static/where.html") : "where do i put this?)";
                        }
                        p(id = "error");
                    }
                    @for (idx, (molecule, appearances)) in molecules::molecules().into_iter().sorted_unstable_by_key(|(_, appearances)| appearances.iter().map(|(_, _, name)| name).min().map(|name| name.to_owned())).enumerate() {
                        div {
                            h2 : appearances.iter().map(|(_, _, name)| name).sorted_unstable().dedup().join("/");
                            : molecule.draw(&format!("product{idx}"));
                        }
                    }
                }
                canvas(id = "next", style = "display: none;");
            }
        }
    }
}

#[rocket::get("/static/common.css")]
fn common_css() -> RawCss<&'static str> {
    RawCss(include_str!("../assets/common.css"))
}

#[rocket::get("/static/common.js")]
fn common_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../assets/common.js"))
}

#[rocket::get("/static/transmogrification.js")]
fn transmogrification_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../assets/transmogrification.js"))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![
        index,
        common_css,
        common_js,
        transmogrification_js,
    ])
}
