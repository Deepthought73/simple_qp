use std::ops;

use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;

impl<T: Into<AffineExpression>> ops::Add<T> for Variable {
    type Output = AffineExpression;

    fn add(self, rhs: T) -> Self::Output {
        rhs.into() + self
    }
}

impl ops::Add<QuadraticExpression> for Variable {
    type Output = QuadraticExpression;

    fn add(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<QuadraticExpression> for AffineExpression {
    type Output = QuadraticExpression;

    fn add(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<&AffineExpression> for AffineExpression {
    type Output = AffineExpression;

    fn add(mut self, rhs: &AffineExpression) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Into<AffineExpression>> ops::Add<T> for AffineExpression {
    type Output = AffineExpression;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs.into();
        self
    }
}

impl ops::Add<&QuadraticExpression> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn add(mut self, rhs: &QuadraticExpression) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Into<QuadraticExpression>> ops::Add<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs.into();
        self
    }
}

//
// Adding to numbers
//

impl ops::Add<Variable> for Float {
    type Output = AffineExpression;

    fn add(self, rhs: Variable) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<Variable> for i32 {
    type Output = AffineExpression;

    fn add(self, rhs: Variable) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<AffineExpression> for Float {
    type Output = AffineExpression;

    fn add(self, rhs: AffineExpression) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<AffineExpression> for i32 {
    type Output = AffineExpression;

    fn add(self, rhs: AffineExpression) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<QuadraticExpression> for Float {
    type Output = QuadraticExpression;

    fn add(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<QuadraticExpression> for i32 {
    type Output = QuadraticExpression;

    fn add(self, rhs: QuadraticExpression) -> Self::Output {
        rhs + self
    }
}

//
// AddAssign
//

impl ops::AddAssign<&AffineExpression> for AffineExpression {
    fn add_assign(&mut self, rhs: &AffineExpression) {
        for (var, factor) in rhs.linear_expression.iter() {
            if let Some(f) = self.linear_expression.get_mut(var) {
                *f += factor;
            } else {
                self.linear_expression.insert(*var, *factor);
            }
        }
        self.constant += rhs.constant;
    }
}

impl<T: Into<AffineExpression>> ops::AddAssign<T> for AffineExpression {
    fn add_assign(&mut self, rhs: T) {
        *self += &rhs.into();
    }
}

impl ops::AddAssign<&QuadraticExpression> for QuadraticExpression {
    fn add_assign(&mut self, rhs: &QuadraticExpression) {
        for (key, factor) in rhs.quadratic_expression.iter() {
            if let Some(f) = self.quadratic_expression.get_mut(key) {
                *f += factor;
            } else {
                self.quadratic_expression.insert(*key, *factor);
            }
        }
        self.affine_expression += &rhs.affine_expression;
    }
}

impl<T: Into<QuadraticExpression>> ops::AddAssign<T> for QuadraticExpression {
    fn add_assign(&mut self, rhs: T) {
        *self += &rhs.into();
    }
}
