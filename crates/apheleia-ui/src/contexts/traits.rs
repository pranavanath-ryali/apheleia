use std::{cell::RefCell, rc::Rc};

use crate::{types::NodeId, world::WorldViewForCommands};

pub trait ContextCommand {
    fn execute(self: Box<Self>, rootnode_data: &mut WorldViewForCommands);
}
