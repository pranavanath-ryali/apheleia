use std::fmt::Debug;

use apheleia_core::types::Vec2;

use crate::{
    runtime_expressions::{Constant, Expr, ExprVec, Expression},
    types::NodeId,
    world::World,
};

/// Core per-node layout data: a node's position and size, both relative to
/// its parent (local) and absolute (global).
///
/// Local `position`/`size` are computed by evaluating `position_expr` and
/// `size_expr` (see [`NodeData::compute_position`] and
/// [`NodeData::compute_size`]) against the [`World`]; `global_position` and
/// `global_size` are derived from these by walking the node's ancestors.
///
/// `NodeData` is kept separate from [`Extension`]s because every node in the
/// framework is expected to have at least a position and size, whereas
/// extensions represent optional, per-use-case data.
pub struct NodeData {
    id: NodeId,
    position_expr: ExprVec,
    size_expr: ExprVec,
    position: Option<Vec2>,
    size: Option<Vec2>,
    global_position: Option<Vec2>,
    global_size: Option<Vec2>,
}
impl Debug for NodeData {
    /// Formats the node's data for debugging, omitting `id` and the
    /// expressions (`position_expr`/`size_expr`) and showing only the
    /// computed position/size values.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeData")
            .field("global_position", &self.global_position)
            .field("global_size", &self.global_size)
            .field("position", &self.position)
            .field("size", &self.size)
            .finish()
    }
}

impl NodeData {
    /// Creates a new [`NodeData`] for the given node, with default (empty)
    /// position/size expressions and all computed values unset.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node this data belongs to.
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            position_expr: Default::default(),
            size_expr: Default::default(),
            position: None,
            size: None,
            global_position: None,
            global_size: None,
        }
    }

    /// Sets the expression used to compute this node's local position.
    pub fn position_expr(&mut self, expr: ExprVec) {
        self.position_expr = expr;
    }
    /// Sets the expression used to compute this node's local size.
    pub fn size_expr(&mut self, expr: ExprVec) {
        self.size_expr = expr;
    }

    /// Sets the node's computed local position.
    pub fn set_position(&mut self, position: Vec2) {
        self.position = Some(position);
    }
    /// Sets the node's computed local size.
    pub fn set_size(&mut self, size: Vec2) {
        self.size = Some(size);
    }

    /// Sets the node's computed global position.
    pub fn set_global_position(&mut self, position: Vec2) {
        self.global_position = Some(position);
    }
    /// Sets the node's computed global size.
    pub fn set_global_size(&mut self, size: Vec2) {
        self.global_size = Some(size);
    }

    /// Returns the node's computed local position, if it has been set.
    pub fn get_position(&self) -> Option<Vec2> {
        self.position
    }
    /// Returns the node's computed local size, if it has been set.
    pub fn get_size(&self) -> Option<Vec2> {
        self.size
    }

    /// Returns the node's computed global position, if it has been set.
    pub fn get_global_position(&self) -> Option<Vec2> {
        self.global_position
    }
    /// Returns the node's computed global size, if it has been set.
    pub fn get_global_size(&self) -> Option<Vec2> {
        self.global_size
    }

    /// Evaluates this node's position expression against the given [`World`],
    /// returning the resulting local position.
    ///
    /// This does not read or write [`NodeData::position`]; use
    /// [`NodeData::set_position`] to store the result if needed.
    pub fn compute_position(&self, world: &World) -> Vec2 {
        self.position_expr.compute_result(self.id, world)
    }
    /// Evaluates this node's size expression against the given [`World`],
    /// returning the resulting local size.
    ///
    /// This does not read or write [`NodeData::size`]; use
    /// [`NodeData::set_size`] to store the result if needed.
    pub fn compute_size(&self, world: &World) -> Vec2 {
        self.size_expr.compute_result(self.id, world)
    }
}
