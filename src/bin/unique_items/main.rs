mod unique_items;

fn main() {
    let input_list : Vec<i32> = vec![1, 4, 1, 5, 5];
    let unique_list = unique_items::exercise::get_unique(&input_list);
    println!("[unique_items::main] Status - {:?}", unique_list);
}


