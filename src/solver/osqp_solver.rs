use osqp::Status;

use crate::constraint::Constraint;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::problem_variables::ProblemVariables;
use crate::solver::{SolvedProblem, Solver, SolverStatus};
use crate::util::{convert_csc_to_osqp_csc_matrix, CscMatrixTripletsBuilder};
use crate::Float;

#[derive(Default)]
pub struct OSQPSolver {
    pub settings: osqp::Settings,
}

impl Solver for OSQPSolver {
    type ObjectiveType = QuadraticExpression;

    fn solve<O: Into<Self::ObjectiveType>>(
        &self,
        problem: ProblemVariables,
        objective: O,
        constraints: Vec<Constraint>,
    ) -> Result<SolvedProblem, SolverStatus> {
        let objective = objective.into();
        let quadratic_objective = objective.quadratic_as_csc(problem.bounds.len());
        let quadratic_objective = convert_csc_to_osqp_csc_matrix(quadratic_objective);
        let linear_objective = objective.linear_as_vector(problem.bounds.len());

        let mut constraint_matrix = CscMatrixTripletsBuilder::new(
            constraints.len() + problem.variable_bound_count(),
            problem.bounds.len(),
        );
        let mut lower_bound = vec![];
        let mut upper_bound = vec![];

        for (row, constraint) in constraints.iter().enumerate() {
            let (lower, upper) = match constraint {
                Constraint::LinearInequality {
                    expression,
                    lower_bound,
                    upper_bound,
                } => (
                    if let Some(lower) = lower_bound {
                        lower - expression.constant
                    } else {
                        Float::NEG_INFINITY
                    },
                    if let Some(upper) = upper_bound {
                        upper - expression.constant
                    } else {
                        Float::INFINITY
                    },
                ),
                Constraint::LinearEquality { expression } => {
                    (-expression.constant, -expression.constant)
                }
            };

            lower_bound.push(lower);
            upper_bound.push(upper);

            match constraint {
                Constraint::LinearInequality { expression, .. }
                | Constraint::LinearEquality { expression } => {
                    for (&var, &factor) in expression.linear_expression.iter() {
                        constraint_matrix.set(row, var, factor);
                    }
                }
            }
        }

        for (variable, (lower, upper)) in problem.bounds.iter().enumerate() {
            if lower.is_some() || upper.is_some() {
                let lower = lower.unwrap_or(Float::NEG_INFINITY);
                let upper = upper.unwrap_or(Float::INFINITY);
                lower_bound.push(lower);
                upper_bound.push(upper);
                constraint_matrix.set(constraints.len() + variable, variable, 1.0);
            }
        }

        let constraint_matrix = convert_csc_to_osqp_csc_matrix(constraint_matrix.build());

        let mut res = osqp::Problem::new(
            quadratic_objective.into_upper_tri(),
            &linear_objective,
            constraint_matrix,
            &lower_bound,
            &upper_bound,
            &self.settings,
        )
        .map_err(|_| SolverStatus::Infeasible)?;

        match res.solve() {
            Status::Solved(x) => Ok(SolvedProblem { x: x.x().to_vec() }),
            Status::PrimalInfeasible(_)
            | Status::PrimalInfeasibleInaccurate(_)
            | Status::DualInfeasible(_)
            | Status::DualInfeasibleInaccurate(_) => Err(SolverStatus::Infeasible),
            _ => Err(SolverStatus::OtherError),
        }
    }
}
