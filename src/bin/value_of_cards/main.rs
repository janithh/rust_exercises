mod value_of_cards;
use value_of_cards::{exercise, Card};

fn main() {
    let input = vec![Card::Two, Card::Three, Card::Ace, Card::Four, Card::Five, Card::Ace, Card::Six, Card::Jack, Card::Seven, Card::Eight, Card::Nine, Card::Queen, Card::King, Card::Ace];

    println!("[value_of_cards::main] Card Value - {}", exercise::get_hand_value(input));
}