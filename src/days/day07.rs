use crate::days::input_helper;

fn get_input() -> Vec<i32> {
    let input: Vec<i32> = input_helper::get_input(7)
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    return input;
}

fn part_1() -> i32 {
    let input = get_input();

    // let example: Vec<i32> = "16,1,2,0,4,2,7,1,2,14" .split(",")
    //     .MAP(|N| N.PARSE::<I32>().UNWRAP())
    //     .COLLECT();

    let max_pos = input.iter().max().unwrap();

    let mut fuel_costs = Vec::new();

    for possible_pos in 0..max_pos.to_owned() {
        let mut fuel_cost = 0;

        for pos in &input {
            let cost = pos - possible_pos;

            // println!("{:?}", cost.abs());
            fuel_cost += cost.abs();
        }

        fuel_costs.push(fuel_cost);
    }

    let cost = fuel_costs.iter().min().unwrap().to_owned();

    println!("Part 1: {:?}", cost);

    return cost;
}

fn part_2() -> i32 {
    let input = get_input();

    // let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
    //     .split(",")
    //     .map(|n| n.parse::<i32>().unwrap())
    //     .collect();

    let max_pos = input.iter().max().unwrap();

    let mut fuel_costs = Vec::new();

    for possible_pos in 0..max_pos.to_owned() {
        let mut fuel_cost = 0;

        for pos in &input {
            let cost = (pos - possible_pos).abs();

            // println!("{:?}", cost.abs());
            // fuel_cost += cost;

            for step in 1..=cost {
                fuel_cost += step * 1
            }
        }

        fuel_costs.push(fuel_cost);
    }

    let cost = fuel_costs.iter().min().unwrap().to_owned();

    println!("Part 2: {:?}", cost);

    return cost;
}

pub fn main() {
    part_1();
    part_2();
}

#[cfg(test)]

mod tests {
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(), 345035);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(), 97038163);
    }
}
