use log::info;

use crate::{runtime_expressions::ExprValue, types::NodeId};

pub struct TerminalWidth;
impl ExprValue for TerminalWidth {
    fn result(&self, id: NodeId, world: &crate::world::World) -> u32 {
        world.terminal_size.x as u32
    }
}

pub struct TerminalHeight;
impl ExprValue for TerminalHeight {
    fn result(&self, id: NodeId, world: &crate::world::World) -> u32 {
        world.terminal_size.y as u32
    }
}

pub struct ParentWidth;
impl ExprValue for ParentWidth {
    fn result(&self, id: NodeId, world: &crate::world::World) -> u32 {
        let parent = world.get_relations().get_ancestor_ids(&id).unwrap()[0];
        world
            .get_nodedata(parent)
            .unwrap()
            .get_size()
            .expect("Parent Size not computed")
            .x
    }
}

pub struct ParentHeight;
impl ExprValue for ParentHeight {
    fn result(&self, id: NodeId, world: &crate::world::World) -> u32 {
        let parent = world.get_relations().get_ancestor_ids(&id).unwrap()[0];
        world
            .get_nodedata(parent)
            .unwrap()
            .get_size()
            .expect("Parent Size not computed")
            .y
    }
}
