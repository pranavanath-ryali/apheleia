use std::os::fd::OwnedFd;

use crate::{
    nodedata::NodeData,
    traits::context_command::ContextCommand,
    types::NodeId,
    utils::{calculate_global_position, calculate_global_size},
};

/// A [`ContextCommand`] that sets the [`NodeData`] for a specific node.
///
/// Wraps a [`NodeId`] and the new [`NodeData`], replacing the node's existing
/// data in the [`World`] when executed.
#[derive(Debug)]
pub struct SetDataForNode(pub NodeId, pub NodeData);

impl SetDataForNode {
    /// Creates a new boxed [`SetDataForNode`] command for the given node and data.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node whose data will be set.
    /// * `data` - The new data to assign to the node.
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}

impl ContextCommand for SetDataForNode {
    /// Executes the command, setting the data for the specified node in the [`World`].
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.set_data(self.0, self.1);
    }
}

/// A [`ContextCommand`] that establishes a parent-child relationship between two nodes.
///
/// Wraps the [`NodeId`]s of a child and its parent, relating them in the
/// [`World`] when executed.
#[derive(Debug)]
pub struct RelateChildWithParent {
    /// The identifier of the parent node.
    pub parent: NodeId,
    /// The identifier of the child node.
    pub child: NodeId,
}

impl RelateChildWithParent {
    /// Creates a new boxed [`RelateChildWithParent`] command for the given child and parent.
    ///
    /// # Arguments
    ///
    /// * `child` - The identifier of the child node.
    /// * `parent` - The identifier of the parent node.
    pub fn new(child: NodeId, parent: NodeId) -> Box<Self> {
        Box::new(Self { child, parent })
    }
}

impl ContextCommand for RelateChildWithParent {
    /// Executes the command, relating the child node with its parent in the [`World`].
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.relate_node_with_parent(self.child, self.parent);
    }
}

/// A [`ContextCommand`] that computes and updates the bounds (position and size)
/// of a specific node.
///
/// Wraps a [`NodeId`], computing its position and size from its [`NodeData`]
/// and the surrounding [`World`], then writing the results back onto the node
/// when executed.
#[derive(Debug)]
pub struct ComputeBoundsForNode(pub NodeId);

impl ComputeBoundsForNode {
    /// Creates a new boxed [`ComputeBoundsForNode`] command for the given node.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node whose bounds will be computed.
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}

impl ContextCommand for ComputeBoundsForNode {
    /// Executes the command, computing the node's position and size and
    /// storing them back onto the node's [`NodeData`] in the [`World`].
    ///
    /// # Panics
    ///
    /// Panics if the node identified by `self.0` does not exist in the `World`.
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        // TODO: Make this better. This was a fuck it, it works solution
        {
            let size = {
                let data = world.get_nodedata(self.0).unwrap();
                data.compute_size(world)
            };
            let data = world.get_nodedata_mut(self.0).unwrap();
            data.set_size(size);
        }

        let position = {
            let data = world.get_nodedata(self.0).unwrap();
            data.compute_position(world)
        };
        let data = world.get_nodedata_mut(self.0).unwrap();
        data.set_position(position);
    }
}

/// A [`ContextCommand`] that computes and updates the global bounds (position
/// and size) of a specific node, accounting for ancestor transforms.
///
/// Wraps a [`NodeId`], computing its global position and size relative to the
/// [`World`]'s node hierarchy, then writing the results back onto the node
/// when executed.
#[derive(Debug)]
pub struct ComputeGlobalBoundsForNode(pub NodeId);
impl ComputeGlobalBoundsForNode {
    /// Creates a new boxed [`ComputeGlobalBoundsForNode`] command for the given node.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node whose global bounds will be computed.
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}

impl ContextCommand for ComputeGlobalBoundsForNode {
    /// Executes the command, computing the node's global position and size
    /// (via [`calculate_global_position`] and [`calculate_global_size`]) and
    /// storing them back onto the node's [`NodeData`] in the [`World`].
    ///
    /// # Panics
    ///
    /// Panics if the node identified by `self.0` does not exist in the `World`.
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        let position = calculate_global_position(world, self.0);
        let size = calculate_global_size(world, self.0);
        let data = world.get_nodedata_mut(self.0).unwrap();
        data.set_global_position(position);
        data.set_global_size(size);
    }
}

#[derive(Debug)]
pub struct ComputeAllBounds;
impl ComputeAllBounds {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl ContextCommand for ComputeAllBounds {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world
            .get_relations()
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|&&id| id != 0)
            .for_each(|&id| {
                let position = {
                    let data = world.get_nodedata(id).unwrap();
                    data.compute_position(world)
                };
                let size = {
                    let data = world.get_nodedata(id).unwrap();
                    data.compute_size(world)
                };

                let data = world.get_nodedata_mut(id).unwrap();
                data.set_position(position);
                data.set_size(size);
            });
    }
}

#[derive(Debug)]
pub struct ComputeAllGlobalBounds;
impl ComputeAllGlobalBounds {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}
impl ContextCommand for ComputeAllGlobalBounds {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world
            .get_relations()
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|&&id| id != 0)
            .for_each(|&id| {
                let position = calculate_global_position(world, id);
                let size = calculate_global_size(world, id);
                let data = world.get_nodedata_mut(id).unwrap();
                data.set_global_position(position);
                data.set_global_size(size);
            });
    }
}
