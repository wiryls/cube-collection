use bevy::prelude::*;

use super::{
    super::{input::MovementChanged, model::World, scene_running::WorldChanged},
    adaption::AutoRescale,
    bundle::Cubic,
    translate::{TranslateColor, TranslatePosition, TranslateShape},
};

pub fn state_system(
    mut commands: Commands,
    mut input_action: EventReader<MovementChanged>,
    mut change_world: EventWriter<WorldChanged>,
    mut query: Query<(Entity, &mut Cubic, &mut AutoRescale)>,
    mut world: ResMut<World>,
    mut ticker: Local<detail::Ticker>,
    mut actions: Local<detail::ActionQueue>,
    mut unchanged: Local<bool>,
    mut completed: Local<bool>,
    time: Res<Time>,
) {
    // update actions
    let any_input = !input_action.is_empty();
    for action in input_action.read() {
        use MovementChanged::*;
        match action {
            Add(m) => actions.add(*m),
            Set(m) => actions.set(*m),
        };
    }

    // ready to change the world!
    if *completed {
        // delay one round to move to next level
        *completed = false;

        // avoid current states affecting next level
        actions.reset();
        ticker.reset();

        // report level change event
        change_world.send(WorldChanged::Next);

        // avoid update completed again
        return;
    }

    // try to update world
    let force = any_input && *unchanged;
    let (diffs, delta) = match ticker.tick(time.delta(), force) {
        false => return, // skip
        true => (world.next(actions.pop()), ticker.remaining()),
    };

    *unchanged = diffs.is_empty();
    if !*unchanged {
        let query = query.iter_mut().filter_map(|(id, cube, position)| {
            diffs.get(&cube.id).map(|diff| (id, cube, position, diff))
        });

        for (id, mut cube, mut position, diff) in query {
            // color
            if let Some(value) = diff.kind {
                let component = TranslateColor::new(cube.kind, value, delta);
                commands.entity(id).insert(component);
                cube.kind = value;
            }

            // shape
            if let Some(value) = diff.neighborhood {
                let component = TranslateShape::new(value);
                commands.entity(id).insert(component);
                cube.neighborhood = value;
            }

            // translation
            if let Some(component) = TranslatePosition::make(&*cube, position.point, diff, delta) {
                commands.entity(id).insert(component);
            }
            if let Some(value) = diff.position {
                position.point = value;
            }
            if let Some(value) = diff.movement {
                cube.movement = value;
            }
            if let Some(value) = diff.constraint {
                cube.constraint = value;
            }
        }

        // check status
        *completed = world.done();
    }
}

mod detail {
    use bevy::{
        prelude::Resource,
        time::{Timer, TimerMode},
    };
    use cube_core::cube::Movement;
    use std::{collections::VecDeque, time::Duration};

    #[derive(Default)]
    pub struct ActionQueue {
        once: VecDeque<Movement>,
        repeat: Option<Movement>,
    }

    impl ActionQueue {
        pub fn add(&mut self, movement: Movement) {
            self.once.push_back(movement);
            self.repeat = Some(movement);
        }

        pub fn set(&mut self, movement: Option<Movement>) {
            match movement {
                None => self.repeat = None,
                Some(movement) => {
                    self.once.clear();
                    self.once.push_back(movement);
                    self.repeat = Some(movement);
                }
            }
        }

        pub fn pop(&mut self) -> Option<Movement> {
            match self.once.pop_front() {
                None => self.repeat,
                Some(movement) => Some(movement),
            }
        }

        pub fn reset(&mut self) {
            self.once.clear();
            self.repeat = None;
        }
    }

    #[derive(Resource)]
    pub struct Ticker(Timer);

    impl Ticker {
        pub fn remaining(&self) -> Duration {
            self.0.remaining()
        }

        pub fn tick(&mut self, delta: Duration, force: bool) -> bool {
            if !force {
                self.0.tick(delta).finished()
            } else {
                self.0.reset();
                true
            }
        }

        pub fn reset(&mut self) {
            self.0.reset()
        }
    }

    impl Default for Ticker {
        fn default() -> Self {
            Self(Timer::new(Duration::from_millis(200), TimerMode::Repeating))
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::detail::*;
    use cube_core::cube::Movement;

    #[test]
    fn action_queue() {
        {
            let mut queue = ActionQueue::default();
            queue.add(Movement::Up);
            queue.set(None);
            assert_eq!(queue.pop(), Some(Movement::Up));
            assert_eq!(queue.pop(), None);
        }
        {
            let mut queue = ActionQueue::default();
            queue.add(Movement::Up);
            queue.set(None);
            queue.add(Movement::Up);
            queue.set(None);
            queue.add(Movement::Left);
            queue.set(None);
            assert_eq!(queue.pop(), Some(Movement::Up));
            assert_eq!(queue.pop(), Some(Movement::Up));
            assert_eq!(queue.pop(), Some(Movement::Left));
            assert_eq!(queue.pop(), None);
        }
    }
}
