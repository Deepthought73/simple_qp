use crate::problem::Problem;
use crate::solver::util::CscMatrixBuilder;
use crate::solver::{SolvedProblem, Solver, SolverStatus};
use crate::Float;
use clarabel::solver::{DefaultSettings, IPSolver, NonnegativeConeT, ZeroConeT};

#[derive(Default)]
pub struct ClarabelSolver {
    pub settings: DefaultSettings<Float>,
}

impl Solver for ClarabelSolver {
    fn solve(&self, problem: Problem) -> Result<SolvedProblem, SolverStatus> {
        let quadratic_objective = problem.objective_to_csc_matrix();
        let linear_objective = problem.linear_objective();

        let mut constraint_matrix =
            CscMatrixBuilder::new(problem.constraints.len(), problem.variable_count);
        let mut b = vec![];

        let mut row = 0;
        for constraint in problem.constraints.iter().filter(|it| it.is_equality) {
            for (&col, &value) in constraint.expression.variables.iter() {
                constraint_matrix.set(row, col, value);
            }
            b.push(-constraint.expression.constant);
            row += 1;
        }
        let equality_count = row;

        for constraint in problem.constraints.iter().filter(|it| !it.is_equality) {
            match (constraint.lower_bound, constraint.upper_bound) {
                (Some(lower), Some(upper)) => {
                    for (&col, &value) in constraint.expression.variables.iter() {
                        constraint_matrix.set(row, col, value);
                    }
                    b.push(upper - constraint.expression.constant);
                    row += 1;
                    for (&col, &value) in constraint.expression.variables.iter() {
                        constraint_matrix.set(row, col, -value);
                    }
                    b.push(-(lower - constraint.expression.constant));
                }
                (None, Some(upper)) => {
                    for (&col, &value) in constraint.expression.variables.iter() {
                        constraint_matrix.set(row, col, value);
                    }
                    b.push(upper - constraint.expression.constant);
                }
                (Some(lower), None) => {
                    for (&col, &value) in constraint.expression.variables.iter() {
                        constraint_matrix.set(row, col, -value);
                    }
                    b.push(-(lower - constraint.expression.constant));
                }
                (_, _) => {}
            }
            row += 1;
        }
        let constraint_matrix = constraint_matrix.build();
        let inequality_count = row - equality_count;

        let cones = vec![
            ZeroConeT(equality_count),
            NonnegativeConeT(inequality_count),
        ];

        println!("{:?}", constraint_matrix.m);
        println!("{:?}", constraint_matrix.n);
        println!("{}", b.len());
        println!("{:?}", cones);

        let mut solver = clarabel::solver::DefaultSolver::new(
            &quadratic_objective,
            &linear_objective,
            &constraint_matrix,
            &b,
            &cones,
            self.settings.clone(),
        );
        solver.solve();

        if solver.solution.status == clarabel::solver::SolverStatus::Solved {
            Ok(SolvedProblem {
                x: solver.solution.x,
            })
        } else {
            // TODO more details
            Err(SolverStatus::Infeasible)
        }
    }
}
