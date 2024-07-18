use crate::constraint::Constraint;
use crate::expressions::variable::Variable;
use crate::Float;
use crate::problem_variables::ProblemVariables;

pub mod clarabel_solver;
pub mod coin_cbc_solver;
pub mod osqp_solver;
pub mod util;

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
    type SolverStatus;

    fn solve<O: Into<Self::ObjectiveType>>(
        &self,
        problem: ProblemVariables,
        objective: O,
        constraints: Vec<Constraint>,
    ) -> Result<SolvedProblem, Self::SolverStatus>;
}
