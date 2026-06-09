use std::marker::PhantomData;

use crate::{systems::system_param_function::SystemParamFunction, world::world_cell::UnsafeWorldCellMut};

/// The trait that is used to store the actual functions.
/// This is used in cases like a Vec of systems, specifically
/// ```rust
/// let mut systems: Vec<Box<dyn System>>
/// ```
/// In the above example, the vector can store multiple types of systems that have number of
/// parameters. A [`System`] is a generic trait used for storage.
pub trait System: 'static {
    fn run(&mut self, world: UnsafeWorldCellMut);
}

/// The actual type that stores the function and the type of system it is
pub struct FunctionSystem<F: SystemParamFunction<Marker>, Marker: 'static> {
    pub func: F,
    pub marker: PhantomData<Marker>,
}
impl<F: SystemParamFunction<Marker>, Marker> FunctionSystem<F, Marker> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            marker: PhantomData,
        }
    }
}
impl<F: SystemParamFunction<Marker>, Marker> System for FunctionSystem<F, Marker> {
    /// Run the system
    fn run(&mut self, world: UnsafeWorldCellMut) {
        self.func.run(world);
    }
}
