use std::fmt::Debug;

use apheleia_core::types::Vec2;

use crate::{
    runtime_expressions::{Constant, Expr, ExprVec, Expression},
    types::NodeId,
    world::World,
};

/// A [`NodeData`] for a specific node stores its position and size relative to the parent and the
/// absolute position and size.
/// This is split from extensions because the framework works on atleast a position and
/// size for a given node.
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeData")
            .field("global_position", &self.global_position)
            .field("global_size", &self.global_size)
            .finish()
    }
}
impl NodeData {
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

    pub fn position_expr(&mut self, expr: ExprVec) {
        self.position_expr = expr;
    }
    pub fn size_expr(&mut self, expr: ExprVec){
        self.size_expr = expr;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = Some(position);
    }
    pub fn set_size(&mut self, size: Vec2) {
        self.size = Some(size);
    }


    pub fn set_global_position(&mut self, position: Vec2) {
        self.global_position = Some(position);
    }
    pub fn set_global_size(&mut self, size: Vec2) {
        self.global_size = Some(size);
    }

    pub fn get_global_position(&self) -> Option<Vec2> {
        self.global_position
    }
    pub fn get_global_size(&self) -> Option<Vec2> {
        self.global_size
    }

    pub fn get_position(&self) -> Option<Vec2> {
        self.position
    }
    pub fn get_size(&self) -> Option<Vec2> {
        self.size
    }

    pub fn compute_position(&self, world: &World) -> Vec2 {
        self.position_expr.compute_result(world)
    }
    pub fn compute_size(&self, world: &World) -> Vec2 {
        self.size_expr.compute_result(world)
    }
}
