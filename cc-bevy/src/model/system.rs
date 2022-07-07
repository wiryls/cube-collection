use super::component::Cubic;
use crate::extra::grid::GridView;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Test;

pub fn movement(
    mut commands: Commands,
    cubes: Query<&mut Cubic>,
    view: ResMut<GridView>,
    time: Res<Time>,
    mut turn: Local<detail::Turn>,
) {
    if turn.tick(time.delta()).finished() {
        let mapper = view.mapping();
    }
}

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
