use std::ops;

use maplit::hashmap;

use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;

impl<T: Into<Float>> ops::Mul<T> for Variable {
    type Output = AffineExpression;

    fn mul(self, factor: T) -> Self::Output {
        AffineExpression {
            linear_expression: hashmap! {
                self.0 => factor.into()
            },
            constant: 0.0,
        }
    }
}

impl<T: Into<Float>> ops::Mul<T> for AffineExpression {
    type Output = AffineExpression;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::Mul<Variable> for Variable {
    type Output = QuadraticExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        QuadraticExpression {
            quadratic_expression: hashmap! {
                [self.0.min(rhs.0), self.0.max(rhs.0)] => 1.0
            },
            affine_expression: Default::default(),
        }
    }
}

impl ops::Mul<Variable> for AffineExpression {
    type Output = QuadraticExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        QuadraticExpression {
            quadratic_expression: self
                .linear_expression
                .into_iter()
                .map(|(k, f)| ([k.min(rhs.0), k.max(rhs.0)], f))
                .collect(),
            affine_expression: AffineExpression {
                linear_expression: hashmap! {
                    rhs.0 => self.constant
                },
                constant: 0.0,
            },
        }
    }
}

impl ops::Mul<AffineExpression> for AffineExpression {
    type Output = QuadraticExpression;

    fn mul(self, rhs: AffineExpression) -> Self::Output {
        let mut ret = QuadraticExpression::default();
        for (k1, f1) in self.linear_expression.iter() {
            for (k2, f2) in rhs.linear_expression.iter() {
                ret += *f1 * *f2 * Variable(*k1) * Variable(*k2);
            }
        }
        ret.affine_expression = rhs.clone() * self.constant
            + self.clone() * rhs.constant
            + self.constant * rhs.constant;
        ret
    }
}

impl<T: Into<Float>> ops::Mul<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

//
// Multiplying to numbers
//

impl ops::Mul<Variable> for Float {
    type Output = AffineExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Variable> for i32 {
    type Output = AffineExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<AffineExpression> for Float {
    type Output = AffineExpression;

    fn mul(self, rhs: AffineExpression) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<AffineExpression> for i32 {
    type Output = AffineExpression;

    fn mul(self, rhs: AffineExpression) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<QuadraticExpression> for Float {
    type Output = QuadraticExpression;

    fn mul(self, rhs: QuadraticExpression) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<QuadraticExpression> for i32 {
    type Output = QuadraticExpression;

    fn mul(self, rhs: QuadraticExpression) -> Self::Output {
        rhs * self
    }
}

//
// MulAssign
//

impl<T: Into<Float>> ops::MulAssign<T> for AffineExpression {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for (_, factor) in self.linear_expression.iter_mut() {
            *factor *= rhs;
        }
        self.constant *= rhs;
    }
}

impl<T: Into<Float>> ops::MulAssign<T> for QuadraticExpression {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for (_, factor) in self.quadratic_expression.iter_mut() {
            *factor *= rhs;
        }
        self.affine_expression *= rhs;
    }
}
