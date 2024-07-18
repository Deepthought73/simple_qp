use crate::constraint::Constraint;
use crate::expressions::affine_expression::AffineExpression;
use crate::problem_variables::ProblemVariables;
use crate::solver::{SolvedProblem, Solver};

#[derive(Default)]
pub struct CoinCbcSolver;

impl Solver for CoinCbcSolver {
    type ObjectiveType = AffineExpression;
    type SolverStatus = coin_cbc::raw::Status;

    fn solve<O: Into<Self::ObjectiveType>>(
        &self,
        problem: ProblemVariables,
        objective: O,
        constraints: Vec<Constraint>,
    ) -> Result<SolvedProblem, Self::SolverStatus> {
        let mut model = coin_cbc::Model::default();

        let x = problem
            .bounds
            .into_iter()
            .map(|(lower, upper)| {
                let col = model.add_col();
                if let Some(lower) = lower {
                    model.set_col_lower(col, lower);
                }
                if let Some(upper) = upper {
                    model.set_col_upper(col, upper);
                }
                col
            })
            .collect::<Vec<_>>();

        let objective = objective.into();
        for (v, c) in objective.linear_expression {
            model.set_obj_coeff(x[v], c);
        }

        for constraint in constraints {
            let row = model.add_row();
            match constraint {
                Constraint::LinearInequality {
                    expression,
                    lower_bound,
                    upper_bound,
                } => {
                    for (v, c) in expression.linear_expression {
                        model.set_weight(row, x[v], c);
                    }
                    if let Some(lower) = lower_bound {
                        model.set_row_lower(row, lower - expression.constant);
                    }
                    if let Some(upper) = upper_bound {
                        model.set_row_upper(row, upper - expression.constant);
                    }
                }
                Constraint::LinearEquality { expression } => {
                    for (v, c) in expression.linear_expression {
                        model.set_weight(row, x[v], c);
                    }
                    model.set_row_equal(row, -expression.constant);
                }
            }
        }

        let sol = model.solve();
        let status = sol.raw().status();
        if status == coin_cbc::raw::Status::Finished || status == coin_cbc::raw::Status::Unlaunched
        {
            if sol.raw().is_continuous_unbounded() || sol.raw().is_proven_infeasible() {
                Err(status)
            } else {
                Ok(SolvedProblem {
                    x: x.into_iter().map(|c| sol.col(c)).collect(),
                })
            }
        } else {
            Err(status)
        }
    }
}
