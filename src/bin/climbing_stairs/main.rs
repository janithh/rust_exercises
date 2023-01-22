mod climbing_stairs;
use climbing_stairs::exercise;

fn main() {
    let steps = 11;

    println!("[climbing_stairs::main] Ways to climb in 1s and 2s - {}", exercise::calculate_ways(steps));
}