use ejercicio_2::file::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename1 = &args[1];
    let filename2 = &args[2];

    let file1 = File::new(filename1);
    let file2 = File::new(filename2);

    file1.print_diff(file2);
}
