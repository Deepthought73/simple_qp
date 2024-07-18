/*
use std::time::Instant;

use array_init::array_init;
use clarabel::algebra::CscMatrix;

use simple_qp::expression::Expression;
use simple_qp::problem_variables::ProblemVariables;

pub fn efficient_loop<const WIDTH: usize>(
    n: usize,
    f: fn([Expression; WIDTH]) -> Expression,
) -> CscMatrix {
    let vars: [Expression; WIDTH] = array_init(|i| Expression::variable(i));
    let row_expr = f(vars);

    let pattern_width = 2 * WIDTH - 1;
    let mut pattern = vec![vec![0.0; pattern_width]; pattern_width];
    for k in 0..WIDTH {
        for i in 0..WIDTH {
            for j in 0..WIDTH {
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

    let mut colptr = vec![0; n + 1];
    let mut rowval = vec![];
    let mut nzval = vec![];

    let mut entry_count = 0;
    for k in 0..WIDTH - 1 {
        for i in 0..pattern_width {
            if pattern[k][i].abs() != 0.0 {
                rowval.push(i);
                nzval.push(pattern[k][i]);
                entry_count += 1;
            }
        }
        colptr[k + 1] = entry_count;
    }

    for k in 0..n - (pattern_width - 1) {
        for j in 0..pattern_width {
            if pattern[WIDTH - 1][j].abs() != 0.0 {
                rowval.push(k + j);
                nzval.push(pattern[WIDTH - 1][j]);
                entry_count += 1;
            }
        }
        colptr[k + WIDTH] = entry_count;
    }

    for k in WIDTH..pattern_width {
        for i in 0..pattern.len() {
            if pattern[k][i].abs() != 0.0 {
                rowval.push(n - pattern_width + i);
                nzval.push(pattern[k][i]);
                entry_count += 1;
            }
        }
        colptr[n - (pattern_width - 1) + k] = entry_count;
    }

    CscMatrix {
        m: n,
        n,
        colptr,
        rowval,
        nzval,
    }
}

fn main() {
    let n = 1000;
    let start = Instant::now();
    let mut problem = ProblemVariables::default();
    let x = problem.add_vec(n, None, None, false);

    for i in 0..n - 2 {
        problem.objective += (2 * x[i].clone() - &x[i + 1] + x[i + 2].clone() * 5.0).square();
    }

    let a = problem.objective_to_csc_matrix();
    println!("{:?}", start.elapsed());

    let start = Instant::now();
    let b = efficient_loop(n, |[x0, x1, x2]| (x0 * 2 - x1 + 5 * x2).square());
    println!("{:?}", start.elapsed());

    println!("{}", (a == b));

    /*
    for i in 0..n {
        for j in 0..n {
            print!("{:5 }", matrix.get_entry((i, j)).unwrap_or(0.0));
        }
        println!()
    }*/
}
*/

use coin_cbc::{Model, Sense};

fn main() {
    let mut model = Model::default();
    model.set_parameter("log", "0");

    let c = model.add_col();

    let r = model.add_row();
    model.set_row_lower(r, 2.0);
    model.set_row_upper(r, 10.0);
    model.set_weight(r, c, 1.0);

    model.set_obj_coeff(c, 1.0);

    model.set_obj_sense(Sense::Minimize);

    let res = model.solve();

    println!("{}", res.col(c));
}
