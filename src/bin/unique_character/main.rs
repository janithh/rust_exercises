mod unique_character;

fn main() {
    let input = String::from("leetcode");
    let index = unique_character::exercise::get_first(input);
    assert_eq!(index, 0);
}


