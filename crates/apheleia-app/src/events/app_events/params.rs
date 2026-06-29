use std::ops::Deref;

use apheleia_core::{KeyEvent, MouseEvent, types::Vec2};
use apheleia_ecs::{systems::system::SystemParam, types::SystemRunStage};

use crate::{events::app_events::AppEvents, types::EventData};

pub struct OnAppEvent<'w> {
    event_data: &'w EventData,
}
impl SystemParam for OnAppEvent<'static> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        if world.current_stage == SystemRunStage::Event {
            return Some(OnAppEvent {
                event_data: &world.get_resource::<AppEvents>().unwrap().event_data,
            });
        }
        None
    }
}
impl<'w> Deref for OnAppEvent<'w> {
    type Target = EventData;

    fn deref(&self) -> &Self::Target {
        self.event_data
    }
}

pub struct OnKeys<'w> {
    key_event: &'w KeyEvent,
}
impl SystemParam for OnKeys<'static> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };

        if world.current_stage != SystemRunStage::Event {
            return None;
        }

        let data = &world.get_resource::<AppEvents>().unwrap().event_data;
        if let EventData::Keys(key_event) = data {
            return Some(OnKeys { key_event });
        }

        None
    }
}
impl<'w> Deref for OnKeys<'w> {
    type Target = KeyEvent;

    fn deref(&self) -> &Self::Target {
        self.key_event
    }
}

pub struct OnMouse<'w> {
    mouse_event: &'w MouseEvent,
}
impl SystemParam for OnMouse<'static> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };

        if world.current_stage != SystemRunStage::Event {
            return None;
        }

        let data = &world.get_resource::<AppEvents>().unwrap().event_data;
        if let EventData::Mouse(mouse_event) = data {
            return Some(OnMouse { mouse_event });
        }

        None
    }
}
impl<'w> Deref for OnMouse<'w> {
    type Target = MouseEvent;

    fn deref(&self) -> &Self::Target {
        self.mouse_event
    }
}

pub struct OnResize<'w> {
    size: &'w Vec2,
}
impl SystemParam for OnResize<'static> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };

        if world.current_stage != SystemRunStage::Event {
            return None;
        }

        let data = &world.get_resource::<AppEvents>().unwrap().event_data;
        if let EventData::Resize(size) = data {
            return Some(OnResize { size });
        }

        None
    }
}
impl<'w> Deref for OnResize<'w> {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        self.size
    }
}
