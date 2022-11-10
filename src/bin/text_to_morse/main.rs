mod text_to_morse;
use text_to_morse::exercise::{convert, get_morse_string};

fn main() {
    let test_string = String::from("Janith");
    let morse_test_string = convert(&test_string);

    println!("Morse Code : {}", get_morse_string(&morse_test_string));   
}