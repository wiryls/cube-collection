use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use super::base::Near;
use super::cube::*;

#[derive(Bundle)]
struct CubeBundle {
    live: Live,
    kind: Type,
}

#[derive(Bundle)]
struct UnitBundle {
    point: GridPoint,
    #[bundle]
    shape: ShapeBundle,
}

pub fn build(unit: f32, ratio: f32, style: Near) -> ShapeBundle {
    let mut points = Vec::with_capacity(12);

    let tests = style.around();
    let max = unit * 0.5;
    let min = max * ratio;

    //    3      2                       0      3
    //     ┌────┬─────────────────────────┬────┐
    //     │    │                         │    │
    //     │    │                         │    │
    //     ├────┼─────────────────────────┼────┤
    //    0│    │1                       1│    │2
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │          (0, 0)         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //     │    │                         │    │
    //    2│    │1                       1│    │0
    //     ├────┼─────────────────────────┼────┤
    //     │    │                         │    │
    //     │    │                         │    │
    //     └────┴─────────────────────────┴────┘
    //    3      0                       2      3
    let mut v = [
        Vec2::new(-min, -max), // 0
        Vec2::new(-min, -min), // 1
        Vec2::new(-max, -min), // 2
        Vec2::new(-max, -max), // 3
    ];
    for i in 0..4 {
        for j in 0..4 {
            (v[j].x, v[j].y) = (v[j].y, -v[j].x);
        }
        match (
            tests[(2 * i + 0)],
            tests[(2 * i + 1)],
            tests[(2 * i + 2) % tests.len()],
        ) {
            (true, true, true) => {
                points.push(v[3]);
            }
            (true, _, true) => {
                points.push(v[0]);
                points.push(v[1]);
                points.push(v[2]);
            }
            (true, _, _) => {
                points.push(v[0]);
            }
            (_, _, true) => {
                points.push(v[2]);
            }
            _ => {
                points.push(v[1]);
            }
        }
    }

    GeometryBuilder::build_as(
        &shapes::Polygon {
            points,
            closed: true,
        },
        DrawMode::Fill(FillMode::color(Color::CYAN)),
        Transform::default(),
    )
}
