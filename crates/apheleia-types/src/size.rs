use num_traits::Num;

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
