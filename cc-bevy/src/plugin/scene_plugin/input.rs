use bevy::{input::keyboard::KeyboardInput, prelude::*};
use cc_core::cube::Movement;
use num_traits::Signed;

use super::Lable;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovementChanged {
    Add(Movement),
    Set(Option<Movement>),
}
impl MovementChanged {
    fn cover(&self, that: &Self) -> bool {
        use MovementChanged::*;
        match self {
            _ if self == that => true,
            Add(_) => false,
            Set(m) => m.map(|movement| *that == Add(movement)).unwrap_or_default(),
        }
    }
}
impl Default for MovementChanged {
    fn default() -> Self {
        Self::Set(None)
    }
}

pub fn setup(app: &mut App) {
    app.add_event::<MovementChanged>().add_system_set(
        SystemSet::new()
            .label(Lable::INPUT)
            .after(Lable::VIEW)
            .with_system(keyboard)
            .into(),
    );
}

#[derive(Default)]
enum Command {
    Movement(MovementChanged),
    #[default]
    DoNothing,
}

fn keyboard(
    mut input: EventReader<KeyboardInput>,
    mut movement_changed: EventWriter<MovementChanged>,
    mut actions: Local<ActionSequence>,
) {
    // try to calculate a command and send it to movement system.
    for (code, key) in input
        .iter()
        .filter_map(|key| key.key_code.map(|code| (code, key)))
    {
        let presse = key.state.is_pressed();
        let output = match code {
            KeyCode::W | KeyCode::Up => actions.input(Movement::Up, presse),
            KeyCode::A | KeyCode::Left => actions.input(Movement::Left, presse),
            KeyCode::S | KeyCode::Down => actions.input(Movement::Down, presse),
            KeyCode::D | KeyCode::Right => actions.input(Movement::Right, presse),
            _ => Command::DoNothing,
        };

        match output {
            Command::Movement(movement) => movement_changed.send(movement),
            Command::DoNothing => {}
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// action sequence

#[derive(Default /* required by Local */)]
struct ActionSequence(Vec<Movement>, MovementChanged);
impl ActionSequence {
    fn input(&mut self, movement: Movement, pressed: bool) -> Command {
        let next = self.update(movement, pressed);
        if !self.1.cover(&next) {
            self.1 = next;
            Command::Movement(self.1.clone())
        } else {
            Command::DoNothing
        }
    }

    fn update(&mut self, movement: Movement, pressed: bool) -> MovementChanged {
        self.0.retain(|&m| m != movement);
        if pressed {
            self.0.push(movement);
        }

        let (conflic, movement) = self.evaluate();
        if conflic {
            MovementChanged::Set(movement)
        } else if let Some(movement) = movement {
            MovementChanged::Add(movement)
        } else {
            MovementChanged::Set(None)
        }
    }

    fn evaluate(&self) -> (bool /* conflic */, Option<Movement> /* result */) {
        let mut x = 0;
        let mut y = 0;

        use Movement::*;
        for &m in self.0.iter() {
            match m {
                Left /*  **/ => x -= 1,
                Down /*  **/ => y += 1,
                Up /*    **/ => y -= 1,
                Right /* **/ => x += 1,
            }
        }
        let conflic = x.abs() + y.abs() != self.0.len() as i32;

        let mut movement = None;
        for &m in self.0.iter().rev() {
            movement = match m {
                Left | Right /* **/ => (x != 0).then_some(m),
                Down | Up /*    **/ => (y != 0).then_some(m),
            };
            if movement.is_some() {
                break;
            }
        }

        (conflic, movement)
    }
}
