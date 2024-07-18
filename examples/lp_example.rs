use simple_qp::constraint;
use simple_qp::problem_variables::ProblemVariables;
use simple_qp::solver::coin_cbc_solver::CoinCbcSolver;
use simple_qp::solver::Solver;

fn main() {
    let mut problem = ProblemVariables::default();
    let x = problem.add_variable(None, Some(1.0));
    let y = problem.add_variable(None, None);
    let z = problem.add_variable(None, None);

    let objective = z;

    let mut constraints = vec![];
    constraints.push(constraint!(y >= 2));
    constraints.push(constraint!(x + z == y));

    let solver = CoinCbcSolver::default();
    let result = solver
        .solve(problem, objective, constraints)
        .expect("Solver error");

    let x = result.value(x);
    let y = result.value(y);
    println!("x = {}", x);
    println!("y = {}", y);
}
