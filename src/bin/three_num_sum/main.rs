mod three_num_sum;

fn main() {
    let values: Vec<i32> = vec![3, 24, 12, 15, 2, 7, 6, 11];
    let sum: i32 = 22;

    match three_num_sum::exercise::get_indexes(values, sum) {
        Some((a, b, c)) => println!("[three_num_sum::main] {}, {}, {}", a, b, c),
        _ => println!("[three_num_sum::main] No match")
    };
}


