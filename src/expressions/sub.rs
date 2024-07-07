use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use std::ops;

impl<T: Into<AffineExpression>> ops::Sub<T> for AffineExpression {
    type Output = AffineExpression;

    fn sub(self, rhs: T) -> Self::Output {
        self + -rhs.into()
    }
}

impl ops::Sub<QuadraticExpression> for Variable {
    type Output = QuadraticExpression;

    fn sub(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + -self
    }
}

impl ops::Sub<QuadraticExpression> for AffineExpression {
    type Output = QuadraticExpression;

    fn sub(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + -self
    }
}

impl<T: Into<AffineExpression>> ops::Sub<T> for Variable {
    type Output = AffineExpression;

    fn sub(self, rhs: T) -> Self::Output {
        self + -rhs.into()
    }
}

impl<T: Into<QuadraticExpression>> ops::Sub<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn sub(self, rhs: T) -> Self::Output {
        self + -rhs.into()
    }
}

//
// Subtracting from numbers
//

impl ops::Sub<Variable> for Float {
    type Output = AffineExpression;

    fn sub(self, rhs: Variable) -> Self::Output {
        -rhs + self
    }
}

impl ops::Sub<Variable> for i32 {
    type Output = AffineExpression;

    fn sub(self, rhs: Variable) -> Self::Output {
        -rhs + self
    }
}

impl ops::Sub<AffineExpression> for Float {
    type Output = AffineExpression;

    fn sub(self, rhs: AffineExpression) -> Self::Output {
        -rhs + self
    }
}

impl ops::Sub<AffineExpression> for i32 {
    type Output = AffineExpression;

    fn sub(self, rhs: AffineExpression) -> Self::Output {
        -rhs + self
    }
}

impl ops::Sub<QuadraticExpression> for Float {
    type Output = QuadraticExpression;

    fn sub(self, rhs: QuadraticExpression) -> Self::Output {
        -rhs + self
    }
}

impl ops::Sub<QuadraticExpression> for i32 {
    type Output = QuadraticExpression;

    fn sub(self, rhs: QuadraticExpression) -> Self::Output {
        -rhs + self
    }
}
