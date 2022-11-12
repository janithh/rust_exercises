mod value_of_cards;
use value_of_cards::{exercise, Card};

fn main() {
    let input = vec![Card::Two, Card::Ace, Card::Five, Card::Ace, Card::Jack, Card::Ace];

    println!("[value_of_cards::main] Card Value - {}", exercise::get_hand_value(input));
}