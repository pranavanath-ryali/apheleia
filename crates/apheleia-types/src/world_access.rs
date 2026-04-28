use std::any::Any;

use crate::{Extension, Resource};

pub trait WorldAccess {
    fn get_extension(&self) -> Option<&dyn Any>;
    fn get_extension_mut(&mut self) -> Option<&mut dyn Any>;

    fn get_resource(&self) -> Option<&dyn Any>;
    fn get_resource_mut(&mut self) -> Option<&mut dyn Any>;
}
