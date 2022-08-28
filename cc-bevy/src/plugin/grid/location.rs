use cc_core::cube::Point;

pub trait Location<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl<T: Copy> Location<T> for (T, T) {
    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }
}

impl<T: Clone> Location<T> for Point<T> {
    fn x(&self) -> T {
        self.x.clone()
    }

    fn y(&self) -> T {
        self.y.clone()
    }
}
