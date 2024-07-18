use clarabel::solver::{DefaultSettings, IPSolver, NonnegativeConeT, ZeroConeT};
use clarabel::solver::SolverStatus::{DualInfeasible, PrimalInfeasible, Solved};

use crate::constraint::Constraint;
use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::Float;
use crate::problem_variables::ProblemVariables;
use crate::solver::{SolvedProblem, Solver, SolverStatus};
use crate::util::CscMatrixTripletsBuilder;

#[derive(Default)]
pub struct ClarabelSolver {
    pub settings: DefaultSettings<Float>,
}

impl Solver for ClarabelSolver {
    type ObjectiveType = QuadraticExpression;

    fn solve<O: Into<Self::ObjectiveType>>(
        &self,
        problem: ProblemVariables,
        objective: O,
        constraints: Vec<Constraint>,
    ) -> Result<SolvedProblem, SolverStatus> {
        let objective = objective.into();
        let quadratic_objective = objective.quadratic_as_csc(problem.bounds.len());
        let linear_objective = objective.linear_as_vector(problem.bounds.len());

        let mut constraint_matrix =
            CscMatrixTripletsBuilder::new(constraints.len(), problem.bounds.len());
        let mut b = vec![];

        let mut row = 0;
        for expression in constraints.iter().filter_map(|it| {
            if let Constraint::LinearEquality { expression } = it {
                Some(expression)
            } else {
                None
            }
        }) {
            for (&col, &value) in expression.linear_expression.iter() {
                constraint_matrix.set(row, col, value);
            }
            b.push(-expression.constant);
            row += 1;
        }
        let equality_count = row;

        for (expression, lower_bound, upper_bound) in constraints.iter().filter_map(|it| {
            if let Constraint::LinearInequality {
                expression,
                lower_bound,
                upper_bound,
            } = it
            {
                Some((expression, lower_bound, upper_bound))
            } else {
                None
            }
        }) {
            if let Some(upper_bound) = upper_bound {
                for (&col, &value) in expression.linear_expression.iter() {
                    constraint_matrix.set(row, col, value);
                }
                b.push(upper_bound - expression.constant);
                row += 1;
            }
            if let Some(lower_bound) = lower_bound {
                for (&col, &value) in expression.linear_expression.iter() {
                    constraint_matrix.set(row, col, -value);
                }
                b.push(-(lower_bound - expression.constant));
                row += 1;
            }
        }
        for (i, (lower, upper)) in problem.bounds.iter().enumerate() {
            if let Some(upper) = upper {
                constraint_matrix.set(row, i, 1.0);
                b.push(*upper);
                row += 1;
            }
            if let Some(lower) = lower {
                constraint_matrix.set(row, i, -1.0);
                b.push(-lower);
                row += 1;
            }
        }
        let constraint_matrix = constraint_matrix.build();
        let inequality_count = row - equality_count;

        let cones = vec![
            ZeroConeT(equality_count),
            NonnegativeConeT(inequality_count),
        ];

        let mut solver = clarabel::solver::DefaultSolver::new(
            &quadratic_objective,
            &linear_objective,
            &constraint_matrix,
            &b,
            &cones,
            self.settings.clone(),
        );
        solver.solve();

        match solver.solution.status {
            Solved => Ok(SolvedProblem {
                x: solver.solution.x,
            }),
            PrimalInfeasible | DualInfeasible => Err(SolverStatus::Infeasible),
            _ => Err(SolverStatus::OtherError),
        }
    }
}
