use std::ops::Deref;

use apheleia_ecs_new::systems::{stages::SystemRunStage, system::SystemParam};
use crossterm::event::KeyEvent;

use crate::{resources::AppEvents, types::{EVENT_FOCUS_GAINED, EVENT_FOCUS_LOST, EVENT_KEYS, EVENT_MOUSE, EVENT_RESIZE, EventData, EventType}};

pub struct OnEvent<'w, const E: usize> {
    data: &'w EventData,
}
impl<'w, const E: usize> OnEvent<'w, E> {
    pub fn new(data: &'w EventData) -> Self {
        Self { data }
    }
}
impl<const E: usize> SystemParam for OnEvent<'static, E> {
    unsafe fn fetch(world: *mut apheleia_ecs_new::world::World) -> Option<Self> {
        let world = unsafe { &mut  *world };

        if world.current_stage != SystemRunStage::Event {
            return None;
        }

        let res = world.get_resource::<AppEvents>().unwrap();
        if res.event_type == EventType::None {
            return None;
        }
        if E == res.event_type.as_usize() {
            return Some(OnEvent::new(&res.data))
        }
        None
    }
}

impl<'w> Deref for OnEvent<'w, { EVENT_KEYS }> {
    type Target = KeyEvent;

    fn deref(&self) -> &Self::Target {
        if let EventData::Keys(key_event) = self.data {
            return key_event
        }
        panic!("Unknown, event not found");
    }
}

pub type OnKeys = OnEvent<'static, { EVENT_KEYS }>;
pub type OnMouse = OnEvent<'static, { EVENT_MOUSE }>;
pub type OnFocusGained = OnEvent<'static, { EVENT_FOCUS_GAINED }>;
pub type OnFocusLost = OnEvent<'static, { EVENT_FOCUS_LOST }>;
pub type OnResize = OnEvent<'static, { EVENT_RESIZE }>;
