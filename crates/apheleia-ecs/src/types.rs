use num_traits::Num;

pub type NodeId = u16;

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

#[derive(Clone)]
pub struct Size<T: Num> {
    pub w: T,
    pub h: T,
}
impl<T: Num> Size<T> {
    pub fn zero() -> Self {
        Self {
            w: T::zero(),
            h: T::zero(),
        }
    }

    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}
