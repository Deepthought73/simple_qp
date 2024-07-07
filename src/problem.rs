use clarabel::algebra::CscMatrix;

use crate::constraint::Constraint;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::solver::util::CscMatrixBuilder;
use crate::solver::{SolvedProblem, Solver, SolverStatus};
use crate::{constraint, Float};

#[derive(Default)]
pub struct Problem {
    pub variable_count: usize,
    pub objective: QuadraticExpression,
    pub constraints: Vec<Constraint>,
}

impl Problem {
    pub fn add_var(&mut self, min: Option<Float>, max: Option<Float>) -> Variable {
        let var = Variable(self.variable_count);
        match (min, max) {
            (Some(lower), Some(upper)) => self.constraints.push(constraint!(lower <= var <= upper)),
            (Some(lower), None) => self.constraints.push(constraint!(lower <= var)),
            (None, Some(upper)) => self.constraints.push(constraint!(var <= upper)),
            (None, None) => {}
        }
        self.variable_count += 1;
        var
    }

    pub fn add_vec(&mut self, n: usize, min: Option<Float>, max: Option<Float>) -> Vec<Variable> {
        (0..n).map(|_| self.add_var(min, max)).collect::<Vec<_>>()
    }

    pub fn solve(self, solver: &dyn Solver) -> Result<SolvedProblem, SolverStatus> {
        solver.solve(self)
    }

    pub fn objective_to_csc_matrix(&self) -> CscMatrix {
        let mut quadratic_objective =
            CscMatrixBuilder::new(self.variable_count, self.variable_count);

        for (k, &factor) in self.objective.quadratic_terms.iter() {
            if k[0] != k[1] {
                quadratic_objective.set(k[0], k[1], factor);
                quadratic_objective.set(k[1], k[0], factor);
            } else {
                quadratic_objective.set(k[0], k[1], 2.0 * factor);
            }
        }

        quadratic_objective.build()
    }

    pub fn linear_objective(&self) -> Vec<Float> {
        let mut linear_objective = vec![0.0; self.variable_count];
        for (&k, factor) in self.objective.linear_expression.variables.iter() {
            linear_objective[k] += factor;
        }
        linear_objective
    }
}
