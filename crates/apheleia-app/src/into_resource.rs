use std::fmt::Debug;

use apheleia_ecs_new::{resources::Resource, world::World};

pub trait IntoResource: Debug {
    fn insert_into(self, world: &mut World);
}

// TODO: Write macro implementation for this
// 1
impl<R1: Resource + 'static> IntoResource for (R1,) {
    fn insert_into(self, world: &mut World) {
        world.add_resource(self.0);
    }
}

// 2
impl<R1: Resource + 'static, R2: Resource + 'static> IntoResource for (R1, R2) {
    fn insert_into(self, world: &mut World) {
        world.add_resource(self.0);
        world.add_resource(self.1);
    }
}

// 3
impl<R1: Resource + 'static, R2: Resource + 'static, R3: Resource + 'static> IntoResource
    for (R1, R2, R3)
{
    fn insert_into(self, world: &mut World) {
        world.add_resource(self.0);
        world.add_resource(self.1);
        world.add_resource(self.2);
    }
}

// 4
impl<R1: Resource + 'static, R2: Resource + 'static, R3: Resource + 'static, R4: Resource + 'static> IntoResource
    for (R1, R2, R3, R4)
{
    fn insert_into(self, world: &mut World) {
        world.add_resource(self.0);
        world.add_resource(self.1);
        world.add_resource(self.2);
        world.add_resource(self.3);
    }
}
