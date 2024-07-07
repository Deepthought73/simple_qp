use clarabel::algebra::CscMatrix;

use crate::Float;

#[derive(Default)]
pub struct CscMatrixBuilder {
    row_count: usize,
    col_count: usize,
    row: Vec<usize>,
    col: Vec<usize>,
    vals: Vec<Float>,
}

impl CscMatrixBuilder {
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

pub fn convert_csc_to_osqp_csc_matrix<'a>(csc_matrix: CscMatrix) -> osqp::CscMatrix<'a> {
    osqp::CscMatrix {
        nrows: csc_matrix.m,
        ncols: csc_matrix.n,
        indptr: csc_matrix.colptr.into(),
        indices: csc_matrix.rowval.into(),
        data: csc_matrix.nzval.into(),
    }
}
