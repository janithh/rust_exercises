#[path = "./bin/calc_median/calc_median.rs"] mod calc_median;

fn main() {
    let mut values: Vec<f64> = vec![1.72];
    let message = match calc_median::exercise::calculate_median(&mut values) {
        Some(_a) => format!("Median - {_a}"),
        None => format!("Median error")
    };
    println!("[rust_exercises::main] calc median : {}", message);
}


