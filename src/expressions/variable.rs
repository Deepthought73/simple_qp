use std::ops;

use maplit::hashmap;

use crate::expressions::affine_expression::AffineExpression;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Variable(pub usize);

impl ops::Neg for Variable {
    type Output = AffineExpression;

    fn neg(self) -> Self::Output {
        AffineExpression {
            linear_expression: hashmap! {
                self.0 => -1.0,
            },
            constant: 0.0,
        }
    }
}
