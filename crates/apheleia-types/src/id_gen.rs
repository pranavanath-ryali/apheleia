use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use num_traits::{Bounded, Num};

pub struct IdGenerator<T> {
    count: T,
    free_ids: Vec<T>,
}
impl<T: Num + Copy + PartialOrd + Display + AddAssign + Bounded> IdGenerator<T> {
    pub fn new(start: T) -> Self {
        Self {
            count: start,
            free_ids: vec![],
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> T {
        if !self.free_ids.is_empty() {
            return self.free_ids.pop().unwrap();
        }

        let id = self.count;
        self.count += T::one();
        id
    }

    pub fn free(&mut self, id: T) {
        self.free_ids.push(id);
    }
}

#[cfg(test)]
mod test_id_generator {
    use crate::id_gen::IdGenerator;

    #[test]
    fn test_id_generator_next() {
        let mut id_generator: IdGenerator<u16> = IdGenerator::new(0);

        assert_eq!(id_generator.next(), 0);
        assert_eq!(id_generator.next(), 1);
        assert_eq!(id_generator.next(), 2);
        assert_eq!(id_generator.next(), 3);
        assert_eq!(id_generator.next(), 4);

        id_generator.free(1);
        assert_eq!(id_generator.next(), 1);
    }
}
