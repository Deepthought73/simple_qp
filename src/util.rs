use array_init::array_init;
use clarabel::algebra::CscMatrix;

use crate::expressions::quadratic_expression::QuadraticExpression;
use crate::expressions::variable::Variable;
use crate::Float;

#[derive(Default)]
pub struct CscMatrixTripletsBuilder {
    row_count: usize,
    col_count: usize,
    row: Vec<usize>,
    col: Vec<usize>,
    vals: Vec<Float>,
}

impl CscMatrixTripletsBuilder {
    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            row_count,
            col_count,
            row: vec![],
            col: vec![],
            vals: vec![],
        }
    }

    pub fn set<T: Into<Float>>(&mut self, row: usize, col: usize, val: T) {
        if row + 1 > self.row_count {
            self.row_count = row + 1;
        }
        if col + 1 > self.col_count {
            self.col_count = col + 1;
        }
        self.row.push(row);
        self.col.push(col);
        self.vals.push(val.into());
    }

    pub fn build(self) -> CscMatrix {
        CscMatrix::new_from_triplets(
            self.row_count,
            self.col_count,
            self.row,
            self.col,
            self.vals,
        )
    }
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
        let vars: [Variable; A] = array_init(Variable);
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

        for (k, pattern_row) in pattern.iter().take(A - 1).enumerate() {
            for (i, &pattern_col) in pattern_row.iter().enumerate() {
                if pattern_col.abs() != 0.0 {
                    self.rowval.push(self.current_row + i);
                    self.nzval.push(pattern_col);
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
