
// Morse code data structure 
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

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

// Trait to convert to morse code
trait MorseCode {
    fn to_morse_code(&self) -> Message; 
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message { 
        let mut morse_message: Message = vec!();
    
        for item in self.chars() {
            let code =  match item {
                'A' | 'a' => vec![Pulse::Short, Pulse::Long],
                'B' | 'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'C' | 'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'D' | 'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'E' | 'e' => vec![Pulse::Short],
                'F' | 'f' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'G' | 'g' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'H' | 'h' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'I' | 'i' => vec![Pulse::Short, Pulse::Short],
                'J' | 'j' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'K' | 'k' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'L' | 'l' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'M' | 'm' => vec![Pulse::Long, Pulse::Long],
                'N' | 'n' => vec![Pulse::Long, Pulse::Short],
                'O' | 'o' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'P' | 'p' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'Q' | 'q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'R' | 'r' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                'S' | 's' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                'T' | 't' => vec![Pulse::Long],
                'U' | 'u' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'V' | 'v' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'W' | 'w' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'X' | 'x' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'Y' | 'y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'Z' | 'z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
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