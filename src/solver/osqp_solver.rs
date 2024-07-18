#[derive(Default)]
pub struct OSQPSolver {
    pub settings: osqp::Settings,
}
/*
impl Solver for OSQPSolver {
    fn solve(&self, problem: Problem) -> Result<SolvedProblem, SolverStatus> {
        let quadratic_objective = convert_csc_to_osqp_csc_matrix(problem.objective_to_csc_matrix());
        let linear_objective = problem.linear_objective();

        let mut constraint_matrix =
            CscMatrixBuilder::new(problem.constraints.len(), problem.variable_count);
        let mut lower_bound = vec![];
        let mut upper_bound = vec![];

        for (row, constraint) in problem.constraints.iter().enumerate() {
            let (lower, upper) = if constraint.is_equality {
                (
                    -constraint.expression.constant,
                    -constraint.expression.constant,
                )
            } else {
                (
                    if let Some(lower) = constraint.lower_bound {
                        lower - constraint.expression.constant
                    } else {
                        Float::NEG_INFINITY
                    },
                    if let Some(upper) = constraint.upper_bound {
                        upper - constraint.expression.constant
                    } else {
                        Float::INFINITY
                    },
                )
            };

            lower_bound.push(lower);
            upper_bound.push(upper);

            for (&var, &factor) in constraint.expression.variables.iter() {
                constraint_matrix.set(row, var, factor);
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

        let status = res.solve();
        if let Some(x) = status.x() {
            Ok(SolvedProblem { x: x.to_vec() })
        } else {
            // TODO match all cases
            Err(SolverStatus::Infeasible)
        }
    }
}
*/
