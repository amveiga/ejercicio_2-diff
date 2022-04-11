use crate::grid::Grid;
use std::fs;

/// Representacion de un archivo con un vector que contiene cada linea y la cantidad de lineas que posee dicho archivo
pub struct File {
    lines_argv: Vec<String>,
    n_lines: usize,
}

impl File {
    /// Constructor del struct File a partir del nombre de un archivo
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let lines_argv: Vec<String> = contents.split('\n').map(str::to_string).collect();

        let n_lines = lines_argv.len();

        File {
            lines_argv,
            n_lines,
        }
    }

    /// Imprime por pantalla la diferencia entre dos archivos
    pub fn print_diff(&self, file_to_compare: File) {
        let grid = Grid::build_lsc_grid(self, &file_to_compare);
        self._print_diff_(
            &file_to_compare,
            grid,
            self.n_lines,
            file_to_compare.n_lines,
        );
    }

    pub fn lines_argv(&self) -> Vec<String> {
        self.lines_argv.clone()
    }

    /// Funcion recursiva que imprie por pantalla la diferencia entre dos archivos utilizando una grilla LCS
    fn _print_diff_(&self, file_to_compare: &File, grid: Grid, i: usize, j: usize) {
        if i > 0 && j > 0 && self.lines_argv[i - 1] == file_to_compare.lines_argv[j - 1] {
            self._print_diff_(file_to_compare, grid, i - 1, j - 1);
            println!("{}", self.lines_argv[i - 1]);
        } else if j > 0 && (i == 0 || grid.matrix()[i][j - 1] >= grid.matrix()[i - 1][j]) {
            self._print_diff_(file_to_compare, grid, i, j - 1);
            println!("> {}", file_to_compare.lines_argv[j - 1]);
        } else if i > 0 && (j == 0 || grid.matrix()[i][j - 1] < grid.matrix()[i - 1][j]) {
            self._print_diff_(file_to_compare, grid, i - 1, j);
            println!("< {}", self.lines_argv[i - 1]);
        } else {
            println!();
        }
    }
}
