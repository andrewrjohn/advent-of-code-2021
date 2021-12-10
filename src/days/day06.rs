use super::input_helper;
use std::collections::HashMap;

fn part_1() -> i32 {
    let conversion_table = HashMap::from([
        (8, "7"),
        (7, "6"),
        (6, "5"),
        (5, "4"),
        (4, "3"),
        (3, "2"),
        (2, "1"),
        (1, "0"),
        (0, "6"),
    ]);
    let mut input = "3,4,3,1,2".replace(",", "");
    // let mut input = input_helper::get_input(6)
    //     .first()
    //     .unwrap()
    //     .replace(",", " ");

    let mut input = "3,4,3,1,2"
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut count = 0;

    // for _ in 0..80 {
    //     let mut zeros = input.iter().filter(|a| a == 0);
    //     let mut non_zeros = input.iter().filter(|a| a != 0);
    // }

    // return 0;

    // for _ in 0..1 {
    //     for i in 0..input.chars().count().clone() {
    //         println!("Input: {}", input);
    //         let age = input.as_bytes()[i] as char;

    //         input = input.replace(age, conversion_table.get(&age).unwrap());

    //         if age == '0' {
    //             input.push_str("8");
    //         }
    //     }
    // }

    // println!("{}", input.chars().count());

    // return input.chars().count() as i32;

    for i in 0..5 {
        for j in 0..input.len() {
            let age = input[j];
            // println!("{}", age);

            if age.to_owned() == 0 {
                let remaining = 5 - i;

                use math::round;
                count += (7 / remaining).floor;
                input[j] = 6;
            } else {
                input[j] = age - 1
            }
        }

        // println!("----");
    }

    println!("{}", count);
    return 0;

    // println!("{}", input.len());

    // return input.len() as i32;
}

pub fn main() {
    part_1();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(), 360761);
    }

    // #[test]
    // fn part_2() {
    //     assert_eq!(super::part_2(), 97038163);
    // }
}
