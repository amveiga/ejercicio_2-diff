use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename1 = &args[1];
    let filename2 = &args[2];

    let file1 = File::new(filename1);
    let file2 = File::new(filename2);

    file1.print_diff(file2);
}

/// Representacion de un archivo con un vector que contiene cada linea y la cantidad de lineas que posee dicho archivo
struct File {
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
        let grid = Grid::new(self, &file_to_compare);
        self._print_diff_(
            &file_to_compare,
            grid,
            self.n_lines,
            file_to_compare.n_lines,
        );
    }

    /// Funcion recursiva que imprie por pantalla la diferencia entre dos archivos utilizando una grilla LCS
    fn _print_diff_(&self, file_to_compare: &File, grid: Grid, i: usize, j: usize) {
        if i > 0 && j > 0 && self.lines_argv[i - 1] == file_to_compare.lines_argv[j - 1] {
            self._print_diff_(file_to_compare, grid, i - 1, j - 1);
            println!("{}", self.lines_argv[i - 1]);
        } else if j > 0 && (i == 0 || grid.matrix[i][j - 1] >= grid.matrix[i - 1][j]) {
            self._print_diff_(file_to_compare, grid, i, j - 1);
            println!("> {}", file_to_compare.lines_argv[j - 1]);
        } else if i > 0 && (j == 0 || grid.matrix[i][j - 1] < grid.matrix[i - 1][j]) {
            self._print_diff_(file_to_compare, grid, i - 1, j);
            println!("< {}", self.lines_argv[i - 1]);
        } else {
            println!();
        }
    }
}

/// Representacion de una grilla LCS
struct Grid {
    matrix: Vec<Vec<i32>>,
}

impl Grid {
    /// Constructor de el struct Grid LCS a partir de dos struct Files
    pub fn new(file1: &File, file2: &File) -> Self {
        let n_rows = file1.lines_argv.len() + 1;
        let n_colums = file2.lines_argv.len() + 1;

        let mut matrix = vec![vec![0; n_colums]; n_rows];

        for i in 0..n_rows - 1 {
            for j in 0..n_colums - 1 {
                if file1.lines_argv[i] == file2.lines_argv[j] {
                    matrix[i + 1][j + 1] = matrix[i][j] + 1;
                } else {
                    matrix[i + 1][j + 1] = cmp::max(matrix[i + 1][j], matrix[i][j + 1]);
                }
            }
        }

        Grid { matrix }
    }
}
