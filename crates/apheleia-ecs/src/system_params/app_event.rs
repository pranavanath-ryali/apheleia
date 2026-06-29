// use std::ops::Deref;
//
// use apheleia_core::{KeyEvent, MouseEvent, types::Vec2};
// use log::info;
//
// use crate::{
//     constants::{EVENT_KEYS, EVENT_MOUSE, EVENT_RESIZE},
//     systems::system::SystemParam,
//     types::{EventData, EventType, SystemRunStage},
// };
//
// pub struct OnAppEvent<'w, const EVENT_TYPE: EventType> {
//     data: &'w EventData,
// }
//
// impl<const EVENT_TYPE: EventType> SystemParam for OnAppEvent<'static, EVENT_TYPE> {
//     unsafe fn fetch<'w>(world: *mut crate::world::World) -> Option<Self> {
//         let world = unsafe { &mut *world };
//
//         assert!(
//             world.current_stage == SystemRunStage::Event,
//             "World is currenly not in `Event` stage. Use the OnAppEvent system param only for systems registered for `Event`"
//         );
//
//         if world.app_event_type == EVENT_TYPE {
//             return Some(OnAppEvent {
//                 data: &world.app_event_data,
//             });
//         }
//         None
//     }
// }
//
// impl<'w> Deref for OnAppEvent<'w, EVENT_KEYS> {
//     type Target = KeyEvent;
//
//     fn deref(&self) -> &Self::Target {
//         if let EventData::Keys(event) = self.data {
//             return event;
//         }
//
//         panic!("Unknown, key event not found");
//     }
// }
// impl<'w> Deref for OnAppEvent<'w, EVENT_MOUSE> {
//     type Target = MouseEvent;
//
//     fn deref(&self) -> &Self::Target {
//         if let EventData::Mouse(event) = self.data {
//             return event;
//         }
//
//         panic!("Unknown, mouse event not found");
//     }
// }
// impl<'w> Deref for OnAppEvent<'w, EVENT_RESIZE> {
//     type Target = Vec2;
//
//     fn deref(&self) -> &Self::Target {
//         if let EventData::Resize(size) = self.data {
//             return size;
//         }
//
//         panic!("Unknown, resize event not found");
//     }
// }
