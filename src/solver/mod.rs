use crate::expressions::variable::Variable;
use crate::problem::Problem;
use crate::Float;

pub mod clarabel_solver;
pub mod osqp_solver;
pub mod util;

#[derive(Debug)]
pub enum SolverStatus {
    Solved,
    Infeasible,
}

pub struct SolvedProblem {
    pub x: Vec<Float>,
}

impl SolvedProblem {
    pub fn eval(&self, var: Variable) -> Float {
        self.x[var.0]
    }

    pub fn eval_vec(&self, vars: Vec<Variable>) -> Vec<Float> {
        vars.iter().map(|i| self.x[i.0]).collect()
    }
}

pub trait Solver {
    fn solve(&self, problem: Problem) -> Result<SolvedProblem, SolverStatus>;
}
