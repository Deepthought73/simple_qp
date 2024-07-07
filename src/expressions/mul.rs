use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use maplit::hashmap;
use std::ops;

impl<T: Into<Float>> ops::Mul<T> for Variable {
    type Output = AffineExpression;

    fn mul(self, factor: T) -> Self::Output {
        AffineExpression {
            variables: hashmap! {
                self.0 => factor.into()
            },
            constant: 0.0,
        }
    }
}

impl<T: Into<Float>> ops::Mul<T> for AffineExpression {
    type Output = AffineExpression;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        for (_, factor) in self.variables.iter_mut() {
            *factor *= rhs;
        }
        self.constant *= rhs;
        self
    }
}

impl ops::Mul<Variable> for Variable {
    type Output = QuadraticExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        QuadraticExpression {
            quadratic_terms: hashmap! {
                [self.0.min(rhs.0), self.0.max(rhs.0)] => 1.0
            },
            linear_expression: Default::default(),
        }
    }
}

impl ops::Mul<Variable> for AffineExpression {
    type Output = QuadraticExpression;

    fn mul(self, rhs: Variable) -> Self::Output {
        QuadraticExpression {
            quadratic_terms: self
                .variables
                .into_iter()
                .map(|(k, f)| ([k.min(rhs.0), k.max(rhs.0)], f))
                .collect(),
            linear_expression: AffineExpression {
                variables: hashmap! {
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
        for (k1, f1) in self.variables.iter() {
            for (k2, f2) in rhs.variables.iter() {
                ret = ret + *f1 * *f2 * Variable(*k1) * Variable(*k2);
            }
        }
        ret.linear_expression = rhs.clone() * self.constant
            + self.clone() * rhs.constant
            + self.constant * rhs.constant;
        ret
    }
}

impl<T: Into<Float>> ops::Mul<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.linear_expression = self.linear_expression * rhs;
        for (_, factor) in self.quadratic_terms.iter_mut() {
            *factor *= rhs;
        }
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
