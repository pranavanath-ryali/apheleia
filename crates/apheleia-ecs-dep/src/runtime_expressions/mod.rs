pub mod values;

use apheleia_core::types::Vec2;
use log::warn;

use crate::{
    types::NodeId,
    world::{self, World},
};

pub trait ExprValue {
    fn result(&self, id: NodeId, world: &World) -> u32;
}

pub struct Constant(pub u32);
impl ExprValue for Constant {
    fn result(&self, _id: NodeId, _world: &World) -> u32 {
        self.0
    }
}

pub enum Expr {
    Value(Box<dyn ExprValue>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

fn compute_expression(expr: &Expr, id: NodeId, world: &World) -> u32 {
    match expr {
        Expr::Value(value) => value.result(id, world),
        Expr::Add(left_expr, right_expr) => {
            compute_expression(left_expr, id, world) + compute_expression(right_expr, id, world)
        }
        Expr::Sub(left_expr, right_expr) => {
            compute_expression(left_expr, id, world) - compute_expression(right_expr, id, world)
        }
        Expr::Divide(left_expr, right_expr) => {
            compute_expression(left_expr, id, world) / compute_expression(right_expr, id, world)
        }
        Expr::Multiply(left_expr, right_expr) => {
            compute_expression(left_expr, id, world) * compute_expression(right_expr, id, world)
        }
    }
}

pub struct Expression(pub Expr);
impl Expression {
    pub fn compute_result(&self, id: NodeId, world: &World) -> u32 {
        compute_expression(&self.0, id, world)
    }
}

pub struct ExprVec {
    pub x: Expression,
    pub y: Expression,
}
impl ExprVec {
    pub fn compute_result(&self, id: NodeId, world: &World) -> Vec2 {
        Vec2 {
            x: self.x.compute_result(id, world),
            y: self.y.compute_result(id, world),
        }
    }
}
impl Default for ExprVec {
    fn default() -> Self {
        Self {
            x: Expression(Expr::Value(Box::new(Constant(0)))),
            y: Expression(Expr::Value(Box::new(Constant(0)))),
        }
    }
}

// #[cfg(test)]
// mod test_expressions {
//     use super::*;
//
//     #[test]
//     fn test_expression_playground() {
//         let world = World::default();
//         let expr = Expression(Expr::Divide(
//             Box::new(Expr::Value(Box::new(Constant(2)))),
//             Box::new(Expr::Value(Box::new(Constant(2)))),
//         ));
//
//         assert_eq!(expr.compute_result(&world), 1);
//     }
// }
