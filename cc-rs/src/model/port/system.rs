use bevy::prelude::*;

pub fn movement(
    mut cubes: Query<(&super::CubeCore, &mut Transform)>,
    time: Res<Time>,
    mut turn: Local<detail::Turn>,
) {
    if turn.tick(time.delta()).finished() {
        // TODO:
    }
}

mod detail {
    use bevy::prelude::*;
    use std::{
        ops::{Deref, DerefMut},
        time::Duration,
    };

    pub struct Turn(pub Timer);

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
