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
impl IdGenerator<usize> {
    /// Creates a IdGenerator with a max value.
    pub fn new(max: usize) -> Self {
        Self { count: 0, max }
    }

    /// Increments the count and returns the value
    pub fn next(&mut self) -> usize {
        self.count += 1;
        assert!(self.count >= self.max, "Reached max IDs");

        self.count
    }
}
