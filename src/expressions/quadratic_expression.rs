use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;

use clarabel::algebra::CscMatrix;

use crate::expressions::affine_expression::AffineExpression;
use crate::expressions::variable::Variable;
use crate::Float;
use crate::solver::util::CscMatrixBuilder;

#[derive(Clone, Debug, Default)]
pub struct QuadraticExpression {
    pub quadratic_expression: HashMap<[usize; 2], Float>,
    pub affine_expression: AffineExpression,
}

impl QuadraticExpression {
    pub fn quadratic_as_csc(&self, variable_count: usize) -> CscMatrix {
        if self.quadratic_expression.is_empty() {
            return CscMatrix::zeros((variable_count, variable_count));
        }

        let mut quadratic_objective = CscMatrixBuilder::new(variable_count, variable_count);

        for (k, &factor) in self.quadratic_expression.iter() {
            if k[0] != k[1] {
                quadratic_objective.set(k[0], k[1], factor);
                quadratic_objective.set(k[1], k[0], factor);
            } else {
                quadratic_objective.set(k[0], k[1], 2.0 * factor);
            }
        }

        quadratic_objective.build()
    }

    pub fn linear_as_vector(&self, variable_count: usize) -> Vec<Float> {
        let mut linear_objective = vec![0.0; variable_count];
        for (&k, factor) in self.affine_expression.linear_expression.iter() {
            linear_objective[k] += factor;
        }
        linear_objective
    }
}

impl Display for QuadraticExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, factor) in self.quadratic_expression.iter() {
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
        write!(f, "{}", self.affine_expression)
    }
}

impl ops::Neg for QuadraticExpression {
    type Output = QuadraticExpression;

    fn neg(mut self) -> Self::Output {
        for (_, f) in self.quadratic_expression.iter_mut() {
            *f = -*f;
        }
        self.affine_expression = -self.affine_expression;
        self
    }
}

impl<T: Into<Float>> From<T> for QuadraticExpression {
    fn from(value: T) -> Self {
        QuadraticExpression {
            quadratic_expression: Default::default(),
            affine_expression: value.into().into(),
        }
    }
}

impl From<Variable> for QuadraticExpression {
    fn from(value: Variable) -> Self {
        QuadraticExpression {
            quadratic_expression: Default::default(),
            affine_expression: value.into(),
        }
    }
}

impl From<AffineExpression> for QuadraticExpression {
    fn from(value: AffineExpression) -> Self {
        QuadraticExpression {
            quadratic_expression: Default::default(),
            affine_expression: value,
        }
    }
}
