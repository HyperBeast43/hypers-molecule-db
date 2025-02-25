use {
    std::{
        collections::{
            HashMap,
            HashSet,
        },
        fmt,
    },
    itertools::Itertools as _,
    omsim_rs::data::*,
};

pub(crate) struct Unparse<'a, T>(pub(crate) &'a T);

impl fmt::Debug for Unparse<'_, Molecule> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Molecule { atoms, bonds } = self.0;
        f.debug_struct("Molecule")
            .field("atoms", &OrderedHashMap(atoms, |k| (k.q, k.r)))
            .field("bonds", &OrderedHashSet(bonds, |b| (b.start.q, b.start.r, b.end.q, b.end.r)))
            .finish()
    }
}

struct OrderedHashMap<'a, T: Ord, K, V, F: Fn(&K) -> T>(&'a HashMap<K, V>, F)
where for<'b> Unparse<'b, K>: fmt::Debug, for<'c> Unparse<'c, V>: fmt::Debug;

impl<'a, T: Ord, K, V, F: Fn(&K) -> T> fmt::Debug for OrderedHashMap<'a, T, K, V, F>
where for<'b> Unparse<'b, K>: fmt::Debug, for<'c> Unparse<'c, V>: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "collect!")?;
        let mut list = f.debug_list();
        for (k, v) in self.0.iter().sorted_by_key(|(k, _)| (self.1)(k)) {
            list.entry(&format_args!("{:?} => {:?}", Unparse(k), Unparse(v)));
        }
        list.finish()
    }
}

struct OrderedHashSet<'a, T: Ord, K, F: Fn(&K) -> T>(&'a HashSet<K>, F)
where for<'b> Unparse<'b, K>: fmt::Debug;

impl<'a, T: Ord, K, F: Fn(&K) -> T> fmt::Debug for OrderedHashSet<'a, T, K, F>
where for<'b> Unparse<'b, K>: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "collect!")?;
        let mut list = f.debug_list();
        for elt in self.0.iter().sorted_by_key(|k| (self.1)(k)) {
            list.entry(&Unparse(elt));
        }
        list.finish()
    }
}

impl fmt::Debug for Unparse<'_, HexIndex> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexIndex { q, r } = self.0;
        f.debug_struct("HexIndex")
            .field("q", q)
            .field("r", r)
            .finish()
    }
}

impl fmt::Debug for Unparse<'_, Atom> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Atom::Salt => write!(f, "Atom::Salt"),
            Atom::Air => write!(f, "Atom::Air"),
            Atom::Earth => write!(f, "Atom::Earth"),
            Atom::Fire => write!(f, "Atom::Fire"),
            Atom::Water => write!(f, "Atom::Water"),
            Atom::Quicksilver => write!(f, "Atom::Quicksilver"),
            Atom::Gold => write!(f, "Atom::Gold"),
            Atom::Silver => write!(f, "Atom::Silver"),
            Atom::Copper => write!(f, "Atom::Copper"),
            Atom::Iron => write!(f, "Atom::Iron"),
            Atom::Tin => write!(f, "Atom::Tin"),
            Atom::Lead => write!(f, "Atom::Lead"),
            Atom::Vitae => write!(f, "Atom::Vitae"),
            Atom::Mors => write!(f, "Atom::Mors"),
            Atom::Repeat => write!(f, "Atom::Repeat"),
            Atom::Quintessence => write!(f, "Atom::Quintessence"),
        }
    }
}

impl fmt::Debug for Unparse<'_, Bond> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Bond { start, end, ty } = self.0;
        f.debug_struct("Bond")
            .field("start", &Unparse(start))
            .field("end", &Unparse(end))
            .field("ty", &Unparse(ty))
            .finish()
    }
}

impl fmt::Debug for Unparse<'_, BondType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            BondType::Normal => write!(f, "BondType::Normal"),
            BondType::Triplex { red, black, yellow } => f.debug_struct("BondType::Triplex")
                .field("red", red)
                .field("black", black)
                .field("yellow", yellow)
                .finish(),
        }
    }
}
