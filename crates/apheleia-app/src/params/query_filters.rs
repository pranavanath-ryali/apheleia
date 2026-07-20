use std::marker::PhantomData;

use apheleia_ecs::{stores::events::{EventRegistry, RenderDirty}, traits::{event_marker::EventMarker, query_filter::QueryFilter}, types::SystemRunStage};

pub struct OnEvent<E: EventMarker> {
    _marker: PhantomData<E>
}
impl<E: EventMarker> QueryFilter for OnEvent<E> {
    fn matches(world: &apheleia_ecs::world::World, id: apheleia_ecs::types::NodeId) -> bool {
        world.get_resource::<EventRegistry>().unwrap().is_local_event::<E>(id)
    }
}

pub struct OnRender;
impl QueryFilter for OnRender {
    fn matches(world: &apheleia_ecs::world::World, id: apheleia_ecs::types::NodeId) -> bool {
        if world.current_stage == SystemRunStage::RenderFlip {
            return true;
        }

        world.get_resource::<EventRegistry>().unwrap().is_local_event::<RenderDirty>(id)
    }
}
