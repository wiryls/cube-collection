use bevy::math::Vec2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Adjacence(u8);

impl Adjacence {
    pub const LEFT /*         **/: Adjacence = Adjacence(0b_10000000);
    pub const LEFT_TOP /*     **/: Adjacence = Adjacence(0b_01000000);
    pub const TOP /*          **/: Adjacence = Adjacence(0b_00100000);
    pub const RIGHT_TOP /*    **/: Adjacence = Adjacence(0b_00010000);
    pub const RIGHT /*        **/: Adjacence = Adjacence(0b_00001000);
    pub const RIGHT_BOTTOM /* **/: Adjacence = Adjacence(0b_00000100);
    pub const BOTTOM /*       **/: Adjacence = Adjacence(0b_00000010);
    pub const LEFT_BOTTOM /*  **/: Adjacence = Adjacence(0b_00000001);
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct CubePattern(pub u8);

#[allow(dead_code)]
impl CubePattern {
    pub const AROUND: [Adjacence; 8] = [
        Adjacence::LEFT,
        Adjacence::LEFT_TOP,
        Adjacence::TOP,
        Adjacence::RIGHT_TOP,
        Adjacence::RIGHT,
        Adjacence::RIGHT_BOTTOM,
        Adjacence::BOTTOM,
        Adjacence::LEFT_BOTTOM,
    ];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, mask: Adjacence) {
        self.0 |= mask.0;
    }

    pub fn unset(&mut self, mask: Adjacence) {
        self.0 &= !mask.0;
    }

    pub fn states(&self) -> [bool; 8] {
        CubePattern::AROUND.map(|mask| (self.0 & mask.0) != 0)
    }

    pub fn boundaries(&self, scale: f32, ratio: f32) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(12);

        let is_occupied = self.states();
        let max = scale * 0.5;
        let min = max * ratio.clamp(0., 1.);

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
}
