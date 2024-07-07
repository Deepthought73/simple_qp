use std::ops;

use crate::expressions::affine_expression::AffineExpression;
use maplit::hashmap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Variable(pub usize);

impl ops::Neg for Variable {
    type Output = AffineExpression;

    fn neg(self) -> Self::Output {
        AffineExpression {
            variables: hashmap! {
                self.0 => -1.0,
            },
            constant: 0.0,
        }
    }
}
