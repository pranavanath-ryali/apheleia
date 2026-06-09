use crate::systems::{
    system::{FunctionSystem, System}, system_param::SystemParam, system_param_function::SystemParamFunction
};

/// The translation that converts a given function to a [`System`] itself.
pub trait IntoSystem<Marker> {
    fn into_system(self) -> Box<dyn System>;
}

impl<F, Marker: SystemParam + 'static> IntoSystem<Marker> for F
where 
    F: SystemParamFunction<Marker> + 'static
{
    fn into_system(self) -> Box<dyn System> {
        Box::new(FunctionSystem::new(self))
    }
}
