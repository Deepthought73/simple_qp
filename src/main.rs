use std::time::Instant;

use array_init::array_init;
use clarabel::algebra::CscMatrix;

use simple_qp::expressions::quadratic_expression::QuadraticExpression;
use simple_qp::expressions::variable::Variable;
use simple_qp::Float;

pub enum Parameter {
    Scalar(Float),
    Vector(Vec<Float>),
}

pub struct SequentialHessianBuilder {
    n: usize,
    current_row: usize,
    entry_count: usize,
    colptr: Vec<usize>,
    rowval: Vec<usize>,
    nzval: Vec<Float>,
}

impl SequentialHessianBuilder {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            current_row: 0,
            entry_count: 0,
            colptr: vec![0; n + 1],
            rowval: vec![],
            nzval: vec![],
        }
    }

    pub fn add_for_loop<const A: usize>(
        &mut self,
        variable_count: usize,
        f: fn([Variable; A]) -> QuadraticExpression,
    ) {
        // TODO add parameters
        let vars: [Variable; A] = array_init(|i| Variable(i));
        let row_expr = f(vars);

        let pattern_width = 2 * A - 1;
        let mut pattern = vec![vec![0.0; pattern_width]; pattern_width];
        for k in 0..A {
            for i in 0..A {
                for j in 0..A {
                    if let Some(value) = row_expr.quadratic_expression.get(&[i, j]) {
                        if i == j {
                            pattern[k + i][k + j] += 2.0 * value;
                        } else {
                            pattern[k + i][k + j] += value;
                            pattern[k + j][k + i] += value;
                        }
                    }
                }
            }
        }

        for k in 0..A - 1 {
            for i in 0..pattern_width {
                if pattern[k][i].abs() != 0.0 {
                    self.rowval.push(self.current_row + i);
                    self.nzval.push(pattern[k][i]);
                    self.entry_count += 1;
                }
            }
            self.colptr[self.current_row + k + 1] = self.entry_count;
        }

        for k in 0..variable_count - (pattern_width - 1) {
            for j in 0..pattern_width {
                if pattern[A - 1][j].abs() != 0.0 {
                    self.rowval.push(self.current_row + k + j);
                    self.nzval.push(pattern[A - 1][j]);
                    self.entry_count += 1;
                }
            }
            self.colptr[self.current_row + k + A] = self.entry_count;
        }

        for k in A..pattern_width {
            for i in 0..pattern.len() {
                if pattern[k][i].abs() != 0.0 {
                    self.rowval
                        .push(self.current_row + variable_count - pattern_width + i);
                    self.nzval.push(pattern[k][i]);
                    self.entry_count += 1;
                }
            }
            self.colptr[self.current_row + variable_count - (pattern_width - 1) + k] =
                self.entry_count;
        }

        self.current_row += variable_count;
    }

    pub fn build(self) -> CscMatrix {
        CscMatrix {
            m: self.n,
            n: self.n,
            colptr: self.colptr,
            rowval: self.rowval,
            nzval: self.nzval,
        }
    }
}

fn main() {
    let n = 1000;
    let start = Instant::now();
    let x = (0..n / 2).map(|i| Variable(i)).collect::<Vec<_>>();
    let y = (0..n / 2).map(|i| Variable(i + n / 2)).collect::<Vec<_>>();

    let mut objective = QuadraticExpression::default();
    for i in 0..n / 2 - 2 {
        objective += (2 * x[i] - x[i + 1] + x[i + 2] * 5.0).square();
    }
    for i in 0..n / 2 - 1 {
        objective += (y[i + 1] - y[i]).square();
    }

    let a = objective.quadratic_as_csc(n);
    println!("{:?}", start.elapsed());

    let start = Instant::now();
    let mut builder = SequentialHessianBuilder::new(n);
    builder.add_for_loop(n / 2, |[x0, x1, x2]| (x0 * 2 - x1 + 5 * x2).square());
    builder.add_for_loop(n / 2, |[x0, x1]| (x1 - x0).square());
    let b = builder.build();
    //let b = efficient_loop(n, |[x0, x1, x2]| (x0 * 2 - x1 + 5 * x2).square());
    println!("{:?}", start.elapsed());

    println!("{}", (a == b));
    /*
    for i in 0..b.n {
        for j in 0..b.n {
            print!("{:5 }", b.get_entry((i, j)).unwrap_or(0.0));
        }
        println!()
    }*/
}
