use std::marker::PhantomData;

use crate::systems::{system::FunctionSystem, system_param_function::SystemParamFunction};

/// The translation that converts a given function to a [`System`] itself.
pub trait IntoSystem<Marker> {
    type System;
    fn into_system(this: Self) -> Self::System;
}

impl<F, Marker: 'static> IntoSystem<Marker> for F
where 
    F: SystemParamFunction<Marker>
{
    type System = FunctionSystem<F, Marker>;

    /// converts the function to [`FunctionSystem`] which implements [`System`]
    fn into_system(this: Self) -> Self::System {
        FunctionSystem {
            func: this,
            marker: PhantomData,
        }
    }
}
