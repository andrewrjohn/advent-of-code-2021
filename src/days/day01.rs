use crate::days::input_helper::{self, StringVecExt};

fn part_1() -> i32 {
    let input = input_helper::get_input(1).to_i32();

    let mut count = 0;
    for (i, measurement) in input.iter().enumerate() {
        match i {
            0 => continue,
            _ => count += if measurement > &input[i - 1] { 1 } else { 0 },
        };
    }

    println!(
        "Part 1 - Measurements that increase from the previous: {}",
        count
    );

    return count;
}

fn part_2() -> i32 {
    let input = input_helper::get_input(1).to_i32();
    let mut count = 0;
    for (i, _) in input.iter().enumerate() {
        if input.len() < i + 4 {
            break;
        }

        let first_sum: i32 = input[i..i + 3].iter().sum();
        let second_sum: i32 = input[i + 1..i + 4].iter().sum();

        count += if second_sum > first_sum { 1 } else { 0 };
    }

    println!(
        "Measurement windows that increase from the previous: {}",
        count
    );
    return count;
}

pub fn main() {
    part_1();

    part_2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(), 1553);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(), 1597);
    }
}
