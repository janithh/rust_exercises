mod square_root;
use square_root::exercise;

fn main() {
    let input = 257;
    let precision = 0.001;

    println!("[square_root::main] Value - {}, Precision - {}, Square root - {}", input, precision, exercise::calculate(input, precision).unwrap_or(-1.0));
}