use crate::constraint::Constraint;
use crate::expressions::variable::Variable;
use crate::problem_variables::ProblemVariables;
use crate::Float;

#[cfg(feature = "clarabel")]
pub mod clarabel_solver;
#[cfg(feature = "coin_cbc")]
pub mod coin_cbc_solver;
#[cfg(feature = "osqp")]
pub mod osqp_solver;

#[derive(Copy, Clone, Debug)]
pub enum SolverStatus {
    Solved,
    Infeasible,
    Unbounded,
    OtherError,
}

pub struct SolvedProblem {
    pub x: Vec<Float>,
}

impl SolvedProblem {
    pub fn value(&self, variable: Variable) -> Float {
        self.x[variable.0]
    }

    pub fn eval_vec(&self, vars: &[Variable]) -> Vec<Float> {
        vars.iter().map(|it| self.x[it.0]).collect()
    }
}

pub trait Solver {
    type ObjectiveType;

    fn solve<O: Into<Self::ObjectiveType>>(
        &self,
        problem: ProblemVariables,
        objective: O,
        constraints: Vec<Constraint>,
    ) -> Result<SolvedProblem, SolverStatus>;
}
