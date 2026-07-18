use crate::{
    commands::ContextCommand,
    nodedata::data::NodeData,
    types::NodeId,
    utils::{calculate_global_position, calculate_global_size},
};

#[derive(Debug)]
pub struct SetDataForNode(pub NodeId, pub NodeData);
impl SetDataForNode {
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}
impl ContextCommand for SetDataForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.set_data(self.0, self.1);
    }
}

#[derive(Debug)]
pub struct RelateChildWithParent {
    pub parent: NodeId,
    pub child: NodeId,
}
impl RelateChildWithParent {
    pub fn new(child: NodeId, parent: NodeId) -> Box<Self> {
        Box::new(Self { child, parent })
    }
}
impl ContextCommand for RelateChildWithParent {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.relate_node_with_parent(self.child, self.parent);
    }
}

#[derive(Debug)]
pub struct ComputeBoundsForNode(pub NodeId);
impl ComputeBoundsForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for ComputeBoundsForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        let position = {
            let data = world.get_nodedata(self.0).unwrap();
            data.compute_position(world)
        };
        let size = {
            let data = world.get_nodedata(self.0).unwrap();
            data.compute_size(world)
        };

        let data = world.get_nodedata_mut(self.0).unwrap();
        data.set_position(position);
        data.set_size(size);
    }
}

#[derive(Debug)]
pub struct ComputeGlobalBoundsForNode(pub NodeId);
impl ComputeGlobalBoundsForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for ComputeGlobalBoundsForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        let position = calculate_global_position(world, self.0);
        let size = calculate_global_size(world, self.0);

        let data = world.get_nodedata_mut(self.0).unwrap();
        data.set_global_position(position);
        data.set_global_size(size);
    }
}

// #[derive(Debug)]
// pub struct CalculateGlobalPositionForNode(pub NodeId);
// #[derive(Debug)]
// pub struct CalculateGlobalSizeForNode(pub NodeId);

// impl CalculateGlobalPositionForNode {
//     pub fn new(id: NodeId) -> Box<Self> {
//         Box::new(Self(id))
//     }
// }
// impl ContextCommand for CalculateGlobalPositionForNode {
//     fn execute(self: Box<Self>, world: &mut crate::world::World) {
//         let position = calculate_global_position(world, self.0);
//         world
//             .get_nodedata_mut(self.0)
//             .expect("Unexpected error trying to unwrap nodedata")
//             .global_position = Some(position);
//     }
// }
//
// impl CalculateGlobalSizeForNode {
//     pub fn new(id: NodeId) -> Box<Self> {
//         Box::new(Self(id))
//     }
// }
// impl ContextCommand for CalculateGlobalSizeForNode {
//     fn execute(self: Box<Self>, world: &mut crate::world::World) {
//         let size = calculate_global_size(world, self.0);
//         world
//             .get_nodedata_mut(self.0)
//             .expect("Unexpected error trying to unwrap nodedata")
//             .global_size = Some(size);
//     }
// }
