mod two_sum;

fn main() {
    let sum = 5;
    let data: Vec<i32> = vec![1, 3, 8, 2, 5]; 
    let indexs = two_sum::exercise::get_indexes(data,sum );
    assert_eq!(indexs, Some(vec![1, 2]));
}