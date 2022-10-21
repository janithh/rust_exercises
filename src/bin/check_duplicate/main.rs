mod check_duplicate;

fn main() {
    let input = vec![1, 7, 3, 5, 6, 4, 7];
    let result = check_duplicate::exercise::contains_duplicate(input);

    assert_eq!(result, true);
}