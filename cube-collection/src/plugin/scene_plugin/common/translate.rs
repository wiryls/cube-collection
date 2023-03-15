use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use cube_core::{
    cube::{Constraint, Kind, Neighborhood, Point},
    Diff,
};

use super::{
    super::{common::style, view::GridView},
    bundle::Cubic,
};

/////////////////////////////////////////////////////////////////////////////
// color

#[derive(Component, Debug)]
pub struct TranslateColor {
    elapse: Timer,
    source: (f32 /* H */, Vec2 /* (S, L) */),
    target: (f32 /* H */, Vec2 /* (S, L) */),
}

impl TranslateColor {
    pub fn new(from: Kind, to: Kind, duration: Duration) -> Self {
        Self {
            elapse: Timer::new(duration, TimerMode::Repeating),
            source: Self::from_kind_to_vec3(from),
            target: Self::from_kind_to_vec3(to),
        }
    }

    fn from_kind_to_vec3(kind: Kind) -> (f32, Vec2) {
        let [h, s, l, _] = style::cube_color(kind).as_hsla_f32();
        (h, [s, l].into())
    }

    fn rotate_to(source: f32, target: f32, limit: f32, percent: f32) -> f32 {
        let delta = target - source;
        let step = delta.min(limit - delta);
        let sign = (limit * 0.5 - delta).signum();
        let mut output = source + sign * step * percent;
        while output > limit {
            output -= limit;
        }
        while output < 0. {
            output += limit;
        }
        output
    }
}

pub fn recolor_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TranslateColor, &mut Fill)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (id, mut translate, mut draw) in &mut query {
        let [h, s, l] = if translate.elapse.tick(delta).finished() {
            commands.entity(id).remove::<TranslateColor>();
            let pair = translate.target;
            [pair.0, pair.1.x, pair.1.y]
        } else {
            let source = translate.source;
            let target = translate.target;
            let percent = translate.elapse.percent();
            let sl = source.1 + (target.1 - source.1) * percent;
            let h = TranslateColor::rotate_to(source.0, target.0, 360., percent);
            [h, sl.x, sl.y]
        };

        let target = Color::hsl(h, s, l);
        draw.color.set_r(target.r());
        draw.color.set_g(target.g());
        draw.color.set_b(target.b());
    }
}

/////////////////////////////////////////////////////////////////////////////
// shape

#[derive(Component, Debug)]
pub struct TranslateShape {
    to: Neighborhood,
}

impl TranslateShape {
    pub fn new(to: Neighborhood) -> Self {
        Self { to }
    }
}

pub fn reshape_system(
    mut commands: Commands,
    mut query: Query<(Entity, &TranslateShape, &mut Path)>,
) {
    for (id, translate, mut path) in &mut query {
        commands.entity(id).remove::<TranslateShape>();
        let points = style::cube_boundaries(translate.to, 0.95);
        let shape = shapes::Polygon {
            points,
            closed: true,
        };
        *path = ShapePath::build_as(&shape);
    }
}

/////////////////////////////////////////////////////////////////////////////
// position

#[derive(Component, Debug)]
pub struct TranslatePosition {
    elapse: Timer,
    parameters: Position,
}

impl TranslatePosition {
    pub fn make(cube: &Cubic, position: Point, diff: &Diff, duration: Duration) -> Option<Self> {
        if let Some(target) = diff.position {
            return Some(TranslatePosition {
                elapse: Timer::new(duration, TimerMode::Once),
                parameters: Position::Move(position, target),
            });
        }

        let movement = diff.movement.unwrap_or(cube.movement);
        let constraint = diff.constraint.unwrap_or(cube.constraint);
        if constraint == Constraint::Stop || movement.is_none() {
            return Some(TranslatePosition {
                elapse: Timer::new(Duration::from_secs(0), TimerMode::Once),
                parameters: Position::Stop(position),
            });
        }

        let movement = movement.unwrap();
        let limit = match constraint {
            Constraint::Slap => 0.5,
            Constraint::Lock => 0.05,
            _ => return None,
        };

        Some(TranslatePosition {
            elapse: Timer::new(duration, TimerMode::Repeating),
            parameters: Position::Spin(position, movement.into(), limit),
        })
    }
}

#[derive(Debug)]
enum Position {
    Move(Point, Point),      // (from, to)
    Spin(Point, Point, f32), // (from, delta, limit)
    Stop(Point),             // (from)
}

pub fn position_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TranslatePosition, &mut Transform)>,
    view: Res<GridView>,
    time: Res<Time>,
) {
    let delta = time.delta();
    let mapper = view.mapping();
    let locate = |o: &Point| (mapper.locate(o) + mapper.scale(&(0.5, 0.5)));

    for (id, mut translate, mut transform) in &mut query {
        let z = transform.translation.z;

        use Position::*;
        if translate.elapse.tick(delta).finished() {
            match translate.parameters {
                Move(_, to) => {
                    transform.translation = locate(&to).extend(z);
                    commands.entity(id).remove::<TranslatePosition>();
                }
                Spin(from, _, _) => {
                    transform.translation = locate(&from).extend(z);
                }
                Stop(from) => {
                    transform.translation = locate(&from).extend(z);
                    commands.entity(id).remove::<TranslatePosition>();
                }
            }
        } else {
            match translate.parameters {
                Move(from, to) => {
                    let percent = translate.elapse.percent();
                    let source = locate(&from);
                    let target = locate(&to);
                    let current = source + (target - source) * percent;
                    transform.translation = current.extend(z);
                }
                Spin(from, delta, limit) => {
                    let percent = translate.elapse.percent();
                    let percent = (1.0 - percent).min(percent).min(limit);
                    let source = locate(&from);
                    let delta = mapper.scale(&delta);
                    let current = source + delta * percent;
                    transform.translation = current.extend(z);
                }
                Stop(from) => {
                    transform.translation = locate(&from).extend(z);
                    commands.entity(id).remove::<TranslatePosition>();
                }
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// fade in and fade out

#[derive(Component, Debug)]
pub struct TranslateAlpha {
    elapse: Timer,
    source: f32,
    target: f32,
}

impl TranslateAlpha {
    pub fn new(from: f32, to: f32, cycle: Duration) -> Self {
        Self {
            elapse: Timer::new(cycle, TimerMode::Repeating),
            source: from,
            target: to,
        }
    }
}

pub fn realpha_system(mut query: Query<(&mut TranslateAlpha, &mut Fill)>, time: Res<Time>) {
    let delta = time.delta();
    for (mut translate, mut draw) in &mut query {
        let alpha = if translate.elapse.tick(delta).finished() {
            translate.source
        } else {
            let percent = (std::f32::consts::PI * translate.elapse.percent()).sin();
            let from = translate.source;
            let to = translate.target;
            from + (to - from) * percent
        };

        draw.color.set_a(alpha);
    }
}
