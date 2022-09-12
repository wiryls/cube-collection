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
    mut actions: Local<detail::ActionQueue>,
    mut completed: Local<bool>,
    time: Res<Time>,
) {
    // update actions
    for action in input_action.iter() {
        use MovementChanged::*;
        match action {
            Add(m) => actions.add(*m),
            Set(m) => actions.set(*m),
        };
    }

    // update world
    let step = world.step();
    let delta = time.delta();
    let diffs = match world.next(delta, || actions.pop()) {
        None => return, // skip
        Some(diffs) => diffs,
    };

    if *completed {
        // delay one round to move to next level
        *completed = false;

        // avoid current actions affecting next level
        actions.reset();

        // change to next level
        change_world.send(WorldChanged::Next);

        // avoid update completed again
        return;
    }

    if !diffs.is_empty() {
        let query = query.iter_mut().filter_map(|(id, cube, position)| {
            diffs.get(&cube.id).map(|diff| (id, cube, position, diff))
        });

        for (id, mut cube, mut position, diff) in query {
            // color
            if let Some(value) = diff.kind {
                let component = TranslateColor::new(cube.kind, value, step);
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
            if let Some(component) = TranslatePosition::make(&*cube, position.point, diff, step) {
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
    use cc_core::cube::Movement;
    use std::collections::VecDeque;

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
            let output = match self.once.pop_front() {
                None => self.repeat,
                Some(movement) => Some(movement),
            };
            output
        }

        pub fn reset(&mut self) {
            self.once.clear();
            self.repeat = None;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::detail::*;
    use cc_core::cube::Movement;

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
