mod sum_with_missing;
use sum_with_missing::exercise;

fn main() {
    let input = vec![Some(1), Some(8), Some(15), Some(7), Some(10)];

    println!("[sum_with_missing::main] Sum - {}", exercise::sum_with_missing(input));
}