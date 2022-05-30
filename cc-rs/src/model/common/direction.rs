#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Direction(u8);

impl Direction {
    pub const LEFT /*         **/ : Direction = Direction(0b_10000000);
    pub const LEFT_TOP /*     **/ : Direction = Direction(0b_01000000);
    pub const TOP /*          **/ : Direction = Direction(0b_00100000);
    pub const RIGHT_TOP /*    **/ : Direction = Direction(0b_00010000);
    pub const RIGHT /*        **/ : Direction = Direction(0b_00001000);
    pub const RIGHT_BOTTOM /* **/ : Direction = Direction(0b_00000100);
    pub const BOTTOM /*       **/ : Direction = Direction(0b_00000010);
    pub const LEFT_BOTTOM /*  **/ : Direction = Direction(0b_00000001);
}
