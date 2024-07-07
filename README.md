# simple_qp

[![Rust](https://github.com/Deepthought73/simple_qp/workflows/Rust/badge.svg)](https://github.com/FlorianNAdam/iprint-rs/actions)

`simple_qp` allows formulating *Quadratic Programming (QP)* problems in a symbolic way.
Define your QP without unreadable matrix initializations.

## Available Solver Backends

At the moment, these are the available solver backends:
- `OSQP`
- `CLARABEL`

## Example Code

```rust
use simple_qp::constraint;
use simple_qp::problem::Problem;
use simple_qp::solver::clarabel_solver::ClarabelSolver;

fn main() {
    let mut problem = Problem::default();
    let x = problem.add_var(Some(0.0), None);
    let y = problem.add_var(Some(0.0), None);

    problem.objective = (x - 42).square() + (y - 73).square() + (x - y).square();

    problem.constraints.push(constraint!(50 <= 1.5 * (x / 3 + 2 * y) <= 100));
    problem.constraints.push(constraint!(x - y == 75 + 2 * y));

    let solver = ClarabelSolver::default();
    let result = problem.solve(&solver).expect("Solver error");

    let x_solution = result.eval(x);
    let y_solution = result.eval(y);

    println!("x = {}, y = {}", x_solution, y_solution);
}
```

## Acknowledgment

Thanks [FlorianNAdam](https://github.com/FlorianNAdam) for the `constraint!` macro.
