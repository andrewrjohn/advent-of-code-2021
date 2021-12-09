use std::cmp::*;

use crate::days::input_helper;

struct Cost {
    fuel: i32,
    position: i32,
}

fn part_1() -> i32 {
    let input: Vec<i32> = input_helper::get_input(7)
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    // let example: Vec<i32> = "16,1,2,0,4,2,7,1,2,14" .split(",")
    //     .map(|n| n.parse::<i32>().unwrap())
    //     .collect();

    let max_pos = input.iter().max().unwrap();

    let mut fuel_costs = Vec::new();

    for possible_pos in 0..max_pos.to_owned() {
        let mut fuel_cost = 0;

        for pos in &input {
            let cost = pos - possible_pos;

            // println!("{:?}", cost.abs());
            fuel_cost += cost.abs();
        }

        fuel_costs.push(Cost {
            fuel: fuel_cost,
            position: possible_pos,
        });
    }

    let cost = fuel_costs.iter().map(|cost| cost.fuel).min().unwrap();

    println!("{:?}", cost);

    return cost;
}

pub fn main() {
    part_1();
}

#[cfg(test)]

mod tests {
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(), 345035);
    }
}
