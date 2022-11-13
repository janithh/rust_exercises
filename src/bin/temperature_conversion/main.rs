mod temperature_conversion;
use temperature_conversion::{Scale, exercise};

fn main() {
    let mut temperature = exercise::create_temperature(38.4, Scale::Celcius);

    println!("[temperature_conversion::main] Temperature - {temperature}");

    exercise::convert(&mut temperature, Scale::Fahrenheit);

    println!("[temperature_conversion::main] Temperature - {temperature}");
}