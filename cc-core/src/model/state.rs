use std::rc::Rc;

use super::{Collection, Collision, DisjointSet, Faction, Movement};

pub struct State {
    active: Collection,
    stable: Rc<Collection>,
    closed: Rc<Collision>,
}

impl State {
    pub fn current() {}

    pub fn diff(&self, that: &Self) /* -> Diff */ {}

    pub fn link(&self) -> Self {
        let mut all = self.active.clone();
        let mut set = DisjointSet::default();
        let faction = Faction::new(all.groups(|x| x.unstable()).map(|x| (x.0, x.2)));
        let faction = detail::Surrounding::new(&self.active, &faction);

        // create set
        all.heads().filter(|x| x.1.unstable()).for_each(|(i, x)| {
            faction
                .edges(x, Movement::Idle)
                .filter(|v| x.absorbable_actively(v.1))
                .for_each(|(u, _)| set.join(i.clone(), u))
        });

        // merge set
        set.groups().into_iter().for_each(|g| all.merge(g));

        Self {
            active: all,
            stable: self.stable.clone(),
            closed: self.closed.clone(),
        }
    }

    pub fn next(&self, movement: Movement) /* -> Self */ {}
}

mod detail {
    use super::super::{Head, HeadID, UnitID};
    use super::{Collection, Faction, Movement};

    pub struct Surrounding<'a>(&'a Collection, &'a Faction);

    impl<'a> Surrounding<'a> {
        pub fn new(collecion: &'a Collection, faction: &'a Faction) -> Self {
            Self(collecion, faction)
        }

        pub fn edges(
            &'a self,
            h: &'a Head,
            m: Movement,
        ) -> impl Iterator<Item = (HeadID, &Head)> + 'a {
            const EMPTY: [UnitID; 0] = [];
            h.edges
                .as_ref()
                .map(|x| x.get(m))
                .unwrap_or(EMPTY.as_slice())
                .into_iter()
                .filter_map(|u| self.0.unit(u).and_then(|u| self.1.get(u.position)))
                .filter_map(|i| self.0.head(&i).map(|h| (i, h)))
        }
    }
}
