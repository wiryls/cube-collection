use crate::model::common::Vicinity;
use bevy::math::Vec2;

pub fn make_boundaries(outer: f32, inner_scale: f32, near: Vicinity) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(12);

    let is_occupied = near.states();
    let max = outer * 0.5;
    let min = max * inner_scale.clamp(0., 1.);

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
            is_occupied[(2 * i + 0)],
            is_occupied[(2 * i + 1)],
            is_occupied[(2 * i + 2) % is_occupied.len()],
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

    points
}
