use std::fmt::Debug;

use apheleia_ecs::resources::Resource;

use crate::types::EventData;

pub struct AppEvents {
    pub event_data: EventData,
}
impl Debug for AppEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppEvents").finish()
    }
}
impl Default for AppEvents {
    fn default() -> Self {
        Self {
            event_data: EventData::None,
        }
    }
}
impl Resource for AppEvents {}
