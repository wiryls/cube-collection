use bevy::prelude::*;
use cc_core::cube::Movement;
use std::collections::VecDeque;

use crate::plugin::scene_plugin::cube::component::GridPoint;

use super::{
    super::{
        super::input::MovementChanged,
        super::scene_running::WorldChanged,
        component::Cubic,
        world::{Input, World},
    },
    translate::{TranslateColor, TranslatePosition, TranslateShape},
};

pub fn state_system(
    mut commands: Commands,
    mut input_action: EventReader<MovementChanged>,
    mut change_world: EventWriter<WorldChanged>,
    mut cubes: Query<(Entity, &mut Cubic, &mut GridPoint)>,
    mut world: ResMut<World>,
    mut actions: Local<ActionQueue>,
    time: Res<Time>,
) {
    // update actions
    for action in input_action.iter() {
        match action {
            MovementChanged::Add(m) => actions.add(*m),
            MovementChanged::Set(m) => actions.set(*m),
        };
    }

    // update world
    let step = world.step();
    let diffs = world.next(time.delta(), &mut *actions);
    if !diffs.is_empty() {
        let query = cubes.iter_mut().filter_map(|(id, cube, position)| {
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

        // check progress
        if world.done() {
            change_world.send(WorldChanged::Next);
        }
    }
}

#[derive(Default)]
pub struct ActionQueue {
    once: VecDeque<Movement>,
    repeat: Option<Movement>,
}

impl ActionQueue {
    fn add(&mut self, movement: Movement) {
        self.once.push_back(movement);
        self.repeat = Some(movement);
    }

    fn set(&mut self, movement: Option<Movement>) {
        match movement {
            None => self.repeat = None,
            Some(movement) => {
                self.once.clear();
                self.once.push_back(movement);
                self.repeat = Some(movement);
            }
        }
    }

    fn pop(&mut self) -> Option<Movement> {
        let output = match self.once.pop_front() {
            None => self.repeat,
            Some(movement) => Some(movement),
        };
        output
    }
}

impl Input for ActionQueue {
    fn fetch(&mut self) -> Option<Movement> {
        self.pop()
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::*;

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
