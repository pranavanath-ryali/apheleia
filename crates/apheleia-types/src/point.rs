use std::ops::{AddAssign, SubAssign};

use num_traits::Num;

#[derive(Clone)]
pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}
impl<T: Num> Point<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: Num + SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
