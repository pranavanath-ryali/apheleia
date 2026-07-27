use apheleia_app::{
    context::node::NodeContext, events::app_events::params::OnMouse, node_definer::NodeDefiner,
};
use apheleia_ecs::{
    constants::STAGE,
    nodedata::NodeData,
    params::{query::Query, resmut::ResMut},
    resources::event_registry::EventRegistry,
    traits::{event_marker::EventMarker, extension::Extension},
    types::{NodeId, SystemRunStage},
};

#[derive(Debug, Default)]
pub struct ButtonWidget;
impl Extension for ButtonWidget {}

impl ButtonWidget {
    pub fn new() -> Self {
        Self
    }
}

impl NodeDefiner for ButtonWidget {
    fn setup(self: Box<Self>, ctx: &mut NodeContext) {
        ctx.add_extension(*self, None);
        ctx.add_system(SystemRunStage::Event, STAGE, button_system);
    }
}

#[derive(Debug)]
pub struct Redraw;
impl EventMarker for Redraw {}

pub fn button_system(
    query: Query<(NodeId, &ButtonWidget, NodeData)>,
    mouse_event: OnMouse,
    mut event_registry: ResMut<EventRegistry>,
) {
    let mouse = &*mouse_event;

    for (id, _btn, data) in query.iter() {
        if let (Some(pos), Some(size)) = (data.get_global_position(), data.get_global_size()) {
            let mx = mouse.column as u32;
            let my = mouse.row as u32;

            if mx >= pos.x && mx < pos.x + size.x && my >= pos.y && my < pos.y + size.y {
                event_registry.add_local_event::<Redraw>(id);
            }
        }
    }
}
