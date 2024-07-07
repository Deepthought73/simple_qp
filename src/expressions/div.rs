use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use maplit::hashmap;
use std::ops;

impl<T: Into<Float>> ops::Div<T> for Variable {
    type Output = AffineExpression;

    fn div(self, rhs: T) -> Self::Output {
        AffineExpression {
            variables: hashmap! {
                self.0 => 1.0 / rhs.into()
            },
            constant: 0.0,
        }
    }
}

impl<T: Into<Float>> ops::Div<T> for AffineExpression {
    type Output = AffineExpression;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Into<Float>> ops::Div<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

//
// DivAssign
//

impl<T: Into<Float>> ops::DivAssign<T> for AffineExpression {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for (_, factor) in self.variables.iter_mut() {
            *factor /= rhs;
        }
    }
}

impl<T: Into<Float>> ops::DivAssign<T> for QuadraticExpression {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for (_, factor) in self.quadratic_terms.iter_mut() {
            *factor /= rhs;
        }
        self.linear_expression /= rhs;
    }
}
