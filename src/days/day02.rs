use crate::days::input_helper::get_input;

pub fn main() {
    let input = get_input(1);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0; // Part 2

    for movement in input {
        let movement: Vec<&str> = movement.split(" ").collect();

        let direction = movement[0];
        let units = movement[1].parse::<i32>().unwrap();

        match direction {
            "up" => {
                // depth -= units; // Part 1
                aim -= units; // Part 2
            }
            "down" => {
                // depth += units; // Part 1
                aim += units; // Part 2
            }
            "forward" => {
                horizontal += units;
                depth += aim * units; // Part 2
            }
            _ => {}
        }
    }

    println!(
        "Horizontal ({}) * Depth ({}) = {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
