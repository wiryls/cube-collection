use bevy::prelude::*;
use cc_core::{
    cube::{Constraint, Movement, Point},
    Diff,
};
use std::time::Duration;

use super::super::{super::view::GridView, component::Cubic};

#[derive(Component)]
pub struct Translate {
    elapse: Timer,
    parameters: Parameters,
}

impl Translate {
    pub fn make(cube: &Cubic, diff: &Diff) -> Option<Self> {
        const DURATION: Duration = Duration::from_millis(200);

        if let Some(position) = diff.position {
            return Some(Translate {
                elapse: Timer::new(DURATION, false),
                parameters: Parameters::Move {
                    source: cube.position,
                    target: position,
                },
            });
        }

        let movement = diff.movement.unwrap_or(cube.movement);
        let constraint = diff.constraint.unwrap_or(cube.constraint);
        if constraint == Constraint::Stop || movement.is_none() {
            return Some(Translate {
                elapse: Timer::new(Duration::from_secs(0), false),
                parameters: Parameters::Stop,
            });
        }

        let movement = movement.unwrap();
        match constraint {
            Constraint::Slap => {
                return Some(Translate {
                    elapse: Timer::new(DURATION, true),
                    parameters: Parameters::Slap {
                        source: cube.position,
                        action: movement,
                    },
                })
            }
            Constraint::Lock => {
                return Some(Translate {
                    elapse: Timer::new(DURATION, true),
                    parameters: Parameters::Lock {
                        source: cube.position,
                        action: movement,
                    },
                })
            }
            _ => None,
        }
    }
}

enum Parameters {
    Move { source: Point, target: Point },
    Slap { source: Point, action: Movement },
    Lock { source: Point, action: Movement },
    Stop,
}

pub fn system(
    mut commands: Commands,
    mut cubes: Query<(Entity, &mut Translate, &mut Transform)>,
    view: Res<GridView>,
    time: Res<Time>,
) {
    let delta = time.delta();
    let mapper = view.mapping();
    for (id, mut translate, mut transform) in cubes.iter_mut() {
        if translate.elapse.tick(delta).finished() {
            match translate.parameters {
                Parameters::Move { source: _, target } => {
                    transform.translation = mapper.absolute(&target).extend(0.);
                    commands.entity(id).remove::<Translate>();
                }
                Parameters::Slap { source, action: _ } => {
                    transform.translation = mapper.absolute(&source).extend(0.);
                }
                Parameters::Lock { source, action: _ } => {
                    transform.translation = mapper.absolute(&source).extend(0.);
                }
                Parameters::Stop => {
                    commands.entity(id).remove::<Translate>();
                }
            }
        } else {
            match translate.parameters {
                Parameters::Move { source: _, target } => {
                    // TODO:
                }
                Parameters::Slap { source, action } => {
                    // TODO:
                }
                Parameters::Lock { source, action } => {
                    // TODO:
                }
                Parameters::Stop => {
                    commands.entity(id).remove::<Translate>();
                }
            }
        }
    }
}
