use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Live;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Clone, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

#[derive(Component, Default)]
pub struct Style(u8);

impl Style {
    pub const LEFT: u8 = 0b_1000_0000;
    pub const LEFT_TOP: u8 = 0b_0100_0000;
    pub const TOP: u8 = 0b_0010_0000;
    pub const RIGHT_TOP: u8 = 0b_0001_0000;
    pub const RIGHT: u8 = 0b_0000_1000;
    pub const RIGHT_BOTTOM: u8 = 0b_0000_0100;
    pub const BOTTOM: u8 = 0b_0000_0010;
    pub const LEFT_BOTTOM: u8 = 0b_0000_0001;

    pub fn set(&mut self, bit: u8) {
        self.0 |= bit;
    }

    pub fn unset(&mut self, bit: u8) {
        self.0 &= !bit;
    }

    pub fn test(&self, bit: u8) -> bool {
        self.0 & bit != 0
    }

    pub fn tests(&self) -> [bool; 8] {
        [
            self.0 & Style::LEFT != 0,
            self.0 & Style::LEFT_TOP != 0,
            self.0 & Style::TOP != 0,
            self.0 & Style::RIGHT_TOP != 0,
            self.0 & Style::RIGHT != 0,
            self.0 & Style::RIGHT_BOTTOM != 0,
            self.0 & Style::BOTTOM != 0,
            self.0 & Style::LEFT_BOTTOM != 0,
        ]
    }
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Move {}

#[derive(Clone, PartialEq)]
pub enum Action {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

#[derive(Bundle)]
struct CubeBundle {
    live: Live,
    kind: Type,
}

#[derive(Bundle)]
struct UnitBundle {
    style: Style,
    point: GridPoint,
    #[bundle]
    shape: ShapeBundle,
}

pub fn build(unit: f32, ratio: f32, style: Style) -> ShapeBundle {
    let mut points = Vec::with_capacity(12);

    let tests = style.tests();
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
    //     │    │                         │    │
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
        (v[0].x, v[0].y) = (v[0].y, -v[0].x);
        (v[1].x, v[1].y) = (v[1].y, -v[1].x);
        (v[2].x, v[2].y) = (v[2].y, -v[2].x);
        (v[3].x, v[3].y) = (v[3].y, -v[3].x);
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
