/// A very simple Id Generator that just increments the count and returns that as the id
/// Example:
/// ```rust
/// let mut generator: IdGenerator<usize> = IdGenerator::new(0);
///
/// assert_eq!(generator.next(), 1);
/// assert_eq!(generator.next(), 2);
/// assert_eq!(generator.next(), 3);
/// ```
pub struct IdGenerator<T> {
    count: T,
}
impl IdGenerator<usize> {
    /// Creates a IdGenerator with a starting value.
    pub fn new(start: usize) -> Self {
        Self {
            count: start
        }
    }

    /// Increments the count and returns the value
    pub fn next(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
