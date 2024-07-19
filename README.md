# simple_qp

[![Rust](https://github.com/Deepthought73/simple_qp/workflows/Rust/badge.svg)](https://github.com/Deepthought73/simple_qp/actions)

`simple_qp` allows formulating *Quadratic Programming (QP)* problems in a symbolic way.
Define your QP without unreadable matrix initializations.

## Available Solver Backends

At the moment, these are the available solver backends:

- `OSQP`
- `CLARABEL`
- `COIN CBC`: restricted to Linear Programming problems

## Example Code

```rust
use simple_qp::constraint;
use simple_qp::problem_variables::ProblemVariables;
use simple_qp::solver::osqp_solver::OSQPSolver;
use simple_qp::solver::Solver;

fn main() {
    let mut problem = ProblemVariables::default();
    let x = problem.add_variable(Some(85.), None);
    let y = problem.add_variable(Some(4.0), None);

    let objective = (x - 42).square() + (y - 73).square() + (x - y).square();

    let constraints = vec![
        constraint!(50 <= 1.5 * (x / 3 + 2 * y) <= 100),
        constraint!(x - y == 75 + 2 * y),
    ];

    let solver = OSQPSolver::default();
    let res = solver
        .solve(problem, objective, constraints)
        .expect("Solver error");

    let x_solution = res.value(x);
    let y_solution = res.value(y);

    println!("x = {}, y = {}", x_solution, y_solution);
}

```

## Acknowledgment

Thanks [FlorianNAdam](https://github.com/FlorianNAdam) for the `constraint!` macro.
