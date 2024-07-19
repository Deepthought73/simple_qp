use std::time::Instant;

use simple_qp::expressions::quadratic_expression::QuadraticExpression;
use simple_qp::expressions::variable::Variable;
use simple_qp::util::SequentialHessianBuilder;

fn main() {
    let n = 1000;
    let start = Instant::now();
    let x = (0..n / 2).map(Variable).collect::<Vec<_>>();
    let y = (0..n / 2).map(|i| Variable(i + n / 2)).collect::<Vec<_>>();

    let mut objective = QuadraticExpression::default();
    for i in 0..n / 2 - 2 {
        objective += (2 * x[i] - x[i + 1] + x[i + 2] * 5.0).square();
    }
    for i in 0..n / 2 - 1 {
        objective += (y[i + 1] - y[i]).square();
    }

    let a = objective.quadratic_as_csc(n);
    println!("normal expressions runtime = {:?}", start.elapsed());

    let start = Instant::now();
    let mut builder = SequentialHessianBuilder::new(n);
    builder.add_for_loop(n / 2, |[x0, x1, x2]| (x0 * 2 - x1 + 5 * x2).square());
    builder.add_for_loop(n / 2, |[y0, y1]| (y1 - y0).square());
    let b = builder.build();
    println!("sequential hessian builder runtime = {:?}", start.elapsed());

    println!("equal: {}", a == b);
}
