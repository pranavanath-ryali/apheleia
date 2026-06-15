use std::fmt::Debug;

use apheleia_ecs::resources::Resource;

use crate::types::{EventData, EventType};

pub struct AppEvents {
    pub event_type: EventType,
    pub data: EventData,
}
impl Debug for AppEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppEvents").finish()
    }
}
impl Resource for AppEvents {}
