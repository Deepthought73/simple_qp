use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Clone, Debug, Default)]
pub struct QuadraticExpression {
    pub quadratic_terms: HashMap<[usize; 2], Float>,
    pub linear_expression: AffineExpression,
}

impl Display for QuadraticExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, factor) in self.quadratic_terms.iter() {
            if k[0] == k[1] {
                if *factor == 1.0 {
                    write!(f, "({})² + ", k[0])?;
                } else if *factor == -1.0 {
                    write!(f, "-({})² + ", k[0])?;
                } else if *factor == 0.0 {
                    continue;
                } else {
                    write!(f, "{} * ({})² + ", factor, k[0])?;
                }
            } else if *factor != 0.0 {
                write!(f, "{} * ({}) * ({}) + ", factor, k[0], k[1])?;
            }
        }
        write!(f, "{}", self.linear_expression)
    }
}

impl ops::Neg for QuadraticExpression {
    type Output = QuadraticExpression;

    fn neg(mut self) -> Self::Output {
        for (_, f) in self.quadratic_terms.iter_mut() {
            *f = -*f;
        }
        self.linear_expression = -self.linear_expression;
        self
    }
}

impl<T: Into<Float>> From<T> for QuadraticExpression {
    fn from(value: T) -> Self {
        QuadraticExpression {
            quadratic_terms: Default::default(),
            linear_expression: value.into().into(),
        }
    }
}

impl From<Variable> for QuadraticExpression {
    fn from(value: Variable) -> Self {
        QuadraticExpression {
            quadratic_terms: Default::default(),
            linear_expression: value.into(),
        }
    }
}

impl From<AffineExpression> for QuadraticExpression {
    fn from(value: AffineExpression) -> Self {
        QuadraticExpression {
            quadratic_terms: Default::default(),
            linear_expression: value,
        }
    }
}
