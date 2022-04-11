use crate::file::File;
use std::cmp;

/// Representacion de una grilla LCS
pub struct Grid {
    matrix: Vec<Vec<i32>>,
}

impl Grid {
    /// Constructor de el struct Grid LCS a partir de dos struct Files
    pub fn build_lsc_grid(file1: &File, file2: &File) -> Self {
        let n_rows = file1.lines_argv().len() + 1;
        let n_colums = file2.lines_argv().len() + 1;

        let mut matrix = vec![vec![0; n_colums]; n_rows];

        for i in 0..n_rows - 1 {
            for j in 0..n_colums - 1 {
                if file1.lines_argv()[i] == file2.lines_argv()[j] {
                    matrix[i + 1][j + 1] = matrix[i][j] + 1;
                } else {
                    matrix[i + 1][j + 1] = cmp::max(matrix[i + 1][j], matrix[i][j + 1]);
                }
            }
        }

        Grid { matrix }
    }

    pub fn matrix(&self) -> Vec<Vec<i32>> {
        self.matrix.clone()
    }
}
