use crate::{commands::WorldCommand, types::traits::Resource};

pub struct InsertResource<T: Resource>(T);

impl<T: Resource> InsertResource<T> {
    pub fn new(res: T) -> Box<Self> {
        Box::new(Self(res))
    }
}

impl<T: Resource> WorldCommand for InsertResource<T> {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.resource_insert(self.0);
    }
}
