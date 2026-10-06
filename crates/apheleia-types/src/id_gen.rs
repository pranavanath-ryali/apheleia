use std::{fmt::Display, ops::{Add, AddAssign}};

use num_traits::{Bounded, Num};

pub struct IdGenerator<T> {
    count: T
}
impl<T: Num + Copy + PartialOrd + Display + AddAssign + Bounded> IdGenerator<T> {
    pub fn new(start: T) -> Self {
        Self { count: start }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> T {
        let id = self.count;
        self.count += T::one();
        id
    }
}

#[cfg(test)]
mod id_generator_test {
    use crate::id_gen::IdGenerator;

    #[test]
    fn test_id_generator_next() {
        let mut id_generator: IdGenerator<u16> = IdGenerator::new(0);

        assert_eq!(id_generator.next(), 0);
        assert_eq!(id_generator.next(), 1);
        assert_eq!(id_generator.next(), 2);
        assert_eq!(id_generator.next(), 3);
        assert_eq!(id_generator.next(), 4);
    }
}
