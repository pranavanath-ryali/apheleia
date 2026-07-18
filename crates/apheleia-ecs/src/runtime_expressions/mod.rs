pub mod values;

use apheleia_core::types::Vec2;

use crate::world::{self, World};

pub trait ExprValue {
    fn result(&self, world: &World) -> u32;
}

pub struct Constant(pub u32);
impl ExprValue for Constant {
    fn result(&self, _world: &World) -> u32 {
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

fn compute_expression(expr: &Expr, world: &World) -> u32 {
    match expr {
        Expr::Value(value) => value.result(world),
        Expr::Add(left_expr, right_expr) => {
            compute_expression(left_expr, world) + compute_expression(right_expr, world)
        }
        Expr::Sub(left_expr, right_expr) => {
            compute_expression(left_expr, world) - compute_expression(right_expr, world)
        }
        Expr::Divide(left_expr, right_expr) => {
            compute_expression(left_expr, world) / compute_expression(right_expr, world)
        }
        Expr::Multiply(left_expr, right_expr) => {
            compute_expression(left_expr, world) * compute_expression(right_expr, world)
        }
    }
}

pub struct Expression(pub Expr);
impl Expression {
    pub fn compute_result(&self, world: &World) -> u32 {
        compute_expression(&self.0, world)
    }
}

pub struct ExprVec {
    pub x: Expression,
    pub y: Expression,
}
impl ExprVec {
    pub fn compute_result(&self, world: &World) -> Vec2 {
        Vec2 {
            x: self.x.compute_result(world),
            y: self.y.compute_result(world),
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
