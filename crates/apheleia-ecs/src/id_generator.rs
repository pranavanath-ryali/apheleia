use std::{fmt::Display, ops::AddAssign};

use log::warn;
use num_traits::Num;

/// A very simple Id Generator that just increments the count and returns that as the id
/// Example:
/// ```rust
/// let mut generator: IdGenerator<usize> = IdGenerator::new(usize::MAX);
///
/// assert_eq!(generator.next(), 1);
/// assert_eq!(generator.next(), 2);
/// assert_eq!(generator.next(), 3);
/// ```
pub struct IdGenerator<T> {
    count: T,
    max: T,
}
impl<T: Num + Copy + PartialOrd + Display + AddAssign> IdGenerator<T> {
    /// Creates a IdGenerator with a max value.
    pub fn new(max: T) -> Self {
        warn!("IDGENERATOR - Created new IdGenerator with max: {}", max);
        Self {
            count: T::zero(),
            max,
        }
    }

    /// Increments the count and returns the value
    pub fn next_id(&mut self) -> T {
        self.count += T::one();
        assert!(self.count < self.max, "Reached max IDs: {}", self.max);

        self.count
    }
}
