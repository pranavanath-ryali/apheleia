pub trait IdGeneratorTrait<T> {
    fn new(start: T) -> Self;
    fn next(&mut self) -> T;
}

pub struct IdGenerator<T> {
    pub count: T,
}
impl Default for IdGenerator<usize> {
    fn default() -> Self {
        IdGenerator { count: 0 }
    }
}
impl IdGeneratorTrait<usize> for IdGenerator<usize> {
    fn new(start: usize) -> Self {
        IdGenerator { count: start }
    }

    fn next(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
