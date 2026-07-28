use apheleia_app::params::{local_events::EventEmitter, query_filters::OnRender};
use apheleia_core::{Color, node_buffer::NodeBuffer, style::Style, types::Vec2};
use apheleia_ecs::{params::query::Query, stores::events::RenderDirty, traits::extension::Extension, types::NodeId};
use log::warn;

#[derive(Debug)]
pub struct BackgroundExtension {
    pub color: Color,
}
impl Extension for BackgroundExtension {}

pub fn render_background(query: Query<(NodeId, &BackgroundExtension, NodeBuffer), OnRender>) {
    for (id, ext, buffer) in query.iter() {
        warn!("[BACKGROUND] Background size: {:?}; color: {:?}", buffer.size, ext.color);
        for y in 0..buffer.size.y {
            buffer.write_string(
                Vec2 { x: 0, y },
                &' '.to_string().repeat(buffer.size.x as usize),
                Some(Style {
                    bg: ext.color,
                    ..Default::default()
                }),
            );
        }
    }
}
