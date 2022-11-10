
type Message = Vec<Letter>;

type Letter = Vec<Pulse>;

pub enum Pulse {
    Short,
    Long
}

impl Pulse {
    fn to_char(&self) -> char {
        match self {
            Self::Short => '.',
            Self::Long => '_'
        }
    }
}

trait MorseCode {
    fn to_morse_code(&self) -> Message; 
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message { 
        let mut morse_message: Message = vec!();
        let upper_string = self.to_uppercase();
    
        for item in upper_string.chars() {
            let code =  match item {
                'A' => vec![Pulse::Short, Pulse::Long],
                'B' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'C' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'D' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'E' => vec![Pulse::Short],
                'F' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'G' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'H' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'I' => vec![Pulse::Short, Pulse::Short],
                'J' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'K' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'L' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'M' => vec![Pulse::Long, Pulse::Long],
                'N' => vec![Pulse::Long, Pulse::Short],
                'O' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'P' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'Q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'R' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                'S' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                'T' => vec![Pulse::Long],
                'U' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'V' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'W' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'X' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'Y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'Z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                _ => vec![]
            };

            if !code.is_empty() {
                morse_message.push(code);
            }
        }

        morse_message
    }
}

pub mod exercise {
    use super::MorseCode;
    use super::Message;

    pub fn get_morse_string(morse_message: &Message) -> String {
        let mut morse_string = String::from("");

        for morse_char in morse_message {  
            for morse_item in morse_char {
                morse_string.push(morse_item.to_char());
            }
            morse_string.push(' ');
        }

        morse_string.pop();

        morse_string
    }

    pub fn convert(str_message: &String) ->  Message {
        str_message.to_morse_code()
    }
}


/*******************
 *  Unit tests
 *******************/
 #[cfg(test)]

mod test {
    use super::exercise::{convert, get_morse_string};

    #[test]
    fn ut_convert_normal_1() {
        let test_string = String::from("Janith");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }

    #[test]
    fn ut_convert_space_front() {
        let test_string = String::from(" Janith");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }

    #[test]
    fn ut_convert_space_end() {
        let test_string = String::from("Janith ");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }

    #[test]
    fn ut_convert_space_middle() {
        let test_string = String::from("Ja nit h");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }

    #[test]
    fn ut_convert_unhandled_char() {
        let test_string = String::from("#Ja-ni$th@");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }

    #[test]
    fn ut_convert_numbers() {
        let test_string = String::from("1Ja2ni8th");
        let morse_test_string = convert(&test_string);

        assert_eq!(get_morse_string(&morse_test_string), String::from(".___ ._ _. .. _ ...."));
    }
}