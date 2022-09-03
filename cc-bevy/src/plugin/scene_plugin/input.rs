use bevy::{input::keyboard::KeyboardInput, prelude::*};
use cc_core::cube::Movement;
use num_traits::Signed;

use super::Lable;

pub fn setup(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
            .label(Lable::INPUT)
            .after(Lable::VIEW)
            .with_system(keyboard)
            .into(),
    );
}

#[derive(Default)]
enum Command {
    AddMovement(Movement),
    SetMovement(Option<Movement>),
    #[default]
    DoNothing,
}

fn keyboard(mut input: EventReader<KeyboardInput>, mut actions: Local<ActionInput>) {
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
            Command::AddMovement(movement) => {
                println!("append {:?}", movement);
            }
            Command::SetMovement(movement) => {
                println!("reset");
                if let Some(movement) = movement {
                    println!("switch {:?}", movement);
                }
            }
            Command::DoNothing => {}
        }
    }

    // todo!()
}

/////////////////////////////////////////////////////////////////////////////
// action

#[derive(Clone, PartialEq, Eq)]
enum CommandMovement {
    Add(Movement),
    Set(Option<Movement>),
}
impl CommandMovement {
    fn cover(&self, that: &Self) -> bool {
        use CommandMovement::*;
        match self {
            _ if self == that => true,
            Add(_) => false,
            Set(movement) => movement
                .map(|movement| *that == Add(movement))
                .unwrap_or_default(),
        }
    }
}
impl Default for CommandMovement {
    fn default() -> Self {
        Self::Set(None)
    }
}
impl From<&CommandMovement> for Command {
    fn from(command: &CommandMovement) -> Self {
        match command {
            CommandMovement::Add(movement) => Command::AddMovement(*movement),
            CommandMovement::Set(movement) => Command::SetMovement(*movement),
        }
    }
}

#[derive(Default /* required by Local */)]
struct ActionInput(Vec<Movement>, CommandMovement);
impl ActionInput {
    fn input(&mut self, movement: Movement, pressed: bool) -> Command {
        let next = self.update(movement, pressed);
        if !self.1.cover(&next) {
            self.1 = next;
            (&self.1).into()
        } else {
            Command::DoNothing
        }
    }

    fn update(&mut self, movement: Movement, pressed: bool) -> CommandMovement {
        self.0.retain(|&m| m != movement);
        if pressed {
            self.0.push(movement);
        }

        let (conflic, movement) = self.evaluate();
        if conflic {
            CommandMovement::Set(movement)
        } else if let Some(movement) = movement {
            CommandMovement::Add(movement)
        } else {
            CommandMovement::Set(None)
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
