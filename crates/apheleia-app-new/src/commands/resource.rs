use std::{mem::{self, take}, ops::DerefMut};

use apheleia_ecs_new::resources::Resource;

use crate::{commands::ContextCommand, into_resource::IntoResource};

#[derive(Debug)]
pub struct AddResource<R: Resource>(pub Box<R>);
// impl<R: Resource + 'static> AddResource<R> {
//     pub fn new(resource: R) -> Box<Self> {
//         Box::new(Self(resource))
//     }
// }
impl<R: Resource> ContextCommand for AddResource<R> {
    fn execute(self: Box<Self>, app: &mut crate::app::App) {
        app.get_world_mut().add_resource_boxed(self.0);
    }
}
