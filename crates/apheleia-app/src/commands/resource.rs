use apheleia_ecs::{command::ContextCommand, resources::Resource, world::World};

#[derive(Debug)]
pub struct AddResource<R: Resource>(pub R);
impl<R: Resource> AddResource<R> {
    pub fn new(res: R) -> Box<Self> {
        Box::new(Self(res))
    }
}
impl<R: Resource> ContextCommand for AddResource<R> {
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_resource(self.0);
    }
}
