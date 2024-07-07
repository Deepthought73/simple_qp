use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use std::ops;

impl<T: Into<AffineExpression>> ops::Add<T> for Variable {
    type Output = AffineExpression;

    fn add(self, rhs: T) -> Self::Output {
        AffineExpression::from(self) + rhs
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

impl<T: Into<AffineExpression>> ops::Add<T> for AffineExpression {
    type Output = AffineExpression;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        for (var, factor) in rhs.variables {
            if let Some(f) = self.variables.get_mut(&var) {
                *f += factor;
            } else {
                self.variables.insert(var, factor);
            }
        }
        self.constant += rhs.constant;
        self
    }
}

impl<T: Into<QuadraticExpression>> ops::Add<T> for QuadraticExpression {
    type Output = QuadraticExpression;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        for (key, factor) in rhs.quadratic_terms {
            if let Some(f) = self.quadratic_terms.get_mut(&key) {
                *f += factor;
            } else {
                self.quadratic_terms.insert(key, factor);
            }
        }
        self.linear_expression = self.linear_expression + rhs.linear_expression;
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
