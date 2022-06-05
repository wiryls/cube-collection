use super::CubeCore;
use crate::extra::grid::GridView;
use crate::model::behavior::Movement;
use crate::model::common::{Collision, DisjointSet};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Test;

pub fn movement(
    mut commands: Commands,
    cubes: Query<(&mut CubeCore, &Children)>,
    view: ResMut<GridView>,
    time: Res<Time>,
    mut turn: Local<detail::Turn>,
) {
    if turn.tick(time.delta()).finished() {
        let mapper = view.mapping();

        let cs: Vec<_> = cubes
            .iter()
            .map(|x| x.0)
            .filter(|c| c.is_active())
            .collect();

        let mut dset = DisjointSet::default();
        let grid = Collision::new(
            cs.iter()
                .enumerate()
                .flat_map(|(i, x)| x.body.units().map(move |x| (x, i))),
        );

        // this: i, x
        // that: u, o
        for (i, x) in cs.iter().enumerate() {
            for u in x.body.edges(Movement::Idle).filter_map(|o| grid.get(&o)) {
                let o = cs[u];
                if x.absorbable(o) && !o.absorbable(x) {
                    dset.join(i, u);
                }
            }
        }

        for group in dset.groups() {
            for i in group {}

            // group
        }
    }
}

fn link(cs: Vec<&mut CubeCore>) {}

mod detail {
    use bevy::prelude::*;
    use std::{
        ops::{Deref, DerefMut},
        time::Duration,
    };

    pub struct Turn(Timer);

    impl Default for Turn {
        fn default() -> Self {
            Self(Timer::new(Duration::from_millis(200), true))
        }
    }

    impl Deref for Turn {
        type Target = Timer;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Turn {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
