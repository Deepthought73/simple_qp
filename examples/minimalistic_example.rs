use simple_qp::constraint;
use simple_qp::problem_variables::ProblemVariables;
use simple_qp::solver::clarabel_solver::ClarabelSolver;
use simple_qp::solver::Solver;

fn main() {
    let mut problem = ProblemVariables::default();
    let x = problem.add_variable(None, None);
    let y = problem.add_variable(None, None);

    let objective = (x - 42).square() + (y - 73).square() + (x - y).square();

    let mut constraints = vec![];
    constraints.push(constraint!(50 <= 1.5 * (x / 3 + 2 * y) <= 100));
    constraints.push(constraint!(x - y == 75 + 2 * y));

    let solver = ClarabelSolver::default();
    let res = solver
        .solve(problem, objective, constraints)
        .expect("Solver error");

    let x_solution = res.value(x);
    let y_solution = res.value(y);

    println!("x = {}, y = {}", x_solution, y_solution);
}
