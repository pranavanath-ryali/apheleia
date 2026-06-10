use apheleia_ecs_new::resources::Resource;

use crate::commands::ContextCommand;

#[derive(Debug)]
pub struct AddResource<R: Resource>(pub R);
impl<R: Resource> AddResource<R> {
    pub fn new(res: R) -> Box<Self> {
        Box::new(Self(res))
    }
}
impl<R: Resource> ContextCommand for AddResource<R> {
    fn execute(self: Box<Self>, app: &mut crate::app::App) {
        app.get_world_mut().add_resource(self.0);
    }
}
