mod check_deadline;
use check_deadline::exercise;

fn main() {
    let event = exercise::create_event("Year end party", (2023, 1, 1, 19, 0, 0));

    if let None = event {
        println!("[check_deadline::main] Event cretae ERROR");
    }
    else {
        println!("[check_deadline::main] Is Active - {}", exercise::check_status(event.unwrap()));
    }
}
