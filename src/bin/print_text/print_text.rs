
/*******************
 *  Exercise
 *******************/
pub mod exercise {
    pub fn info<T: std::fmt::Display>(text: &T) {
            println!("[print_text::exercise::info] Text - {}", text);
    }
}

/*******************
 *  Unit tests
 *******************/
#[cfg(test)]

mod test {
    use super::exercise;

    #[test]
    fn ut_info_literal() {
        let input = "Unit Test";
        exercise::info(&input);
    }

    #[test]
    fn ut_info_string() {
        let input = String::from("Unit Test");
        exercise::info(&input);       
    }
}