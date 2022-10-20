
mod print_text;

fn main() {
    let a: &str = "Test";
    let b: String = String::from(a);

    print_text::exercise::info(&a);
    print_text::exercise::info(&b);
}