mod top_k_frequent;

fn main() {
    let input = vec![1, 5, 1, 1, 5, 4, 4, 5, 4, 5, 6];
    let k = 2;
    
    let output = top_k_frequent::exercise::top_k_frequent(input, k);

    println!("[top_k_frequent::main] Result - {:?}", output);
}