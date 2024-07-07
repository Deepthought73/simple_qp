use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;

use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use maplit::hashmap;

#[derive(Clone, Debug, Default)]
pub struct AffineExpression {
    pub variables: HashMap<usize, Float>,
    pub constant: Float,
}

impl AffineExpression {
    pub fn square(self) -> QuadraticExpression {
        self.clone() * self
    }
}

impl Display for AffineExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, factor) in self.variables.iter() {
            if *factor == 1.0 {
                write!(f, "({}) + ", k)?;
            } else if *factor == -1.0 {
                write!(f, "-({}) + ", k)?;
            } else if *factor == 0.0 {
                continue;
            } else {
                write!(f, "{} * ({}) + ", factor, k)?;
            }
        }
        write!(f, "{}", self.constant)
    }
}

impl ops::Neg for AffineExpression {
    type Output = AffineExpression;

    fn neg(mut self) -> Self::Output {
        for (_, factors) in self.variables.iter_mut() {
            *factors = -*factors;
        }
        self.constant = -self.constant;
        self
    }
}

impl<T: Into<Float>> From<T> for AffineExpression {
    fn from(value: T) -> Self {
        AffineExpression {
            variables: Default::default(),
            constant: value.into(),
        }
    }
}

impl From<Variable> for AffineExpression {
    fn from(value: Variable) -> Self {
        AffineExpression {
            variables: hashmap! { value.0 => 1.0 },
            constant: 0.0,
        }
    }
}
