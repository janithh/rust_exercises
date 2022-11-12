
const HAND_LIMIT: usize = 21;

#[derive(PartialEq)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace
}

impl Card {
    fn value(&self) -> usize {
        match self {
            Card:: Two  => 2,
            Card::Three => 3,
            Card::Four  => 4,
            Card::Five  => 5,
            Card::Six   => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine  => 9,
            Card::Jack | Card::Queen | Card::King => 10,
            Card::Ace   => 11
        }
    } 
}

pub struct Hand {
    _cards: Vec<Card>,
    value: usize
}

impl Hand {
    fn new(input: Vec<Card>) -> Self {
        let mut ace_count = 0;
        let mut hand_value = 0;
        
        for card in &input {
            hand_value += card.value();

            if *card == Card::Ace {
                ace_count += 1;               
            }
        }

        println!("{hand_value}, {ace_count}");

        if hand_value > HAND_LIMIT {
            while ace_count != 0 {

                if hand_value < HAND_LIMIT {
                    break;
                }
                else {
                    hand_value -= 10;
                }

                ace_count -= 1;
            }
        }


        Hand {
            _cards: input,
            value: hand_value
        }
    }

    fn value(&self) -> usize {
        self.value
    } 
}

/*******************
 *  Exercise
 *******************/
 pub mod exercise {
    use super::{Card, Hand};

    pub fn get_hand_value(input: Vec<Card>) -> usize {
        let hand = Hand::new(input);
        hand.value()
    }
 }

 /*******************
 *  Unit tests
 *******************/
#[cfg(test)]
mod test {
    use super::{exercise, Card};

    #[test]
    fn ut_hand_21_less_no_ace() {
        let input = vec![Card::Two, Card::Five, Card::Three, Card::Jack];

        assert_eq!(exercise::get_hand_value(input), 20);
    }

    #[test]
    fn ut_hand_21_more_one_ace() {
        let input = vec![Card::Two, Card::Four, Card::Three, Card::Queen, Card::Ace];

        assert_eq!(exercise::get_hand_value(input), 20);
    }

    #[test]
    fn ut_hand_21_more_three_aces() {
        let input = vec![Card::Two, Card::Ace, Card::Five, Card::Ace, Card::King, Card::Ace];

        assert_eq!(exercise::get_hand_value(input), 20);
    }

    #[test]
    fn ut_hand_one_ace() {
        let input = vec![Card::Ace];

        assert_eq!(exercise::get_hand_value(input), 11);
    }

    #[test]
    fn ut_hand_five_aces() {
        let input = vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];

        assert_eq!(exercise::get_hand_value(input), 15);
    }
}