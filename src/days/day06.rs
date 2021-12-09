use super::input_helper;

fn part_1() -> i32 {
    // let input = "3,4,3,1,2";
    let mut input = input_helper::get_input(6)
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    for _ in 0..80 {
        for i in 0..input.len() {
            let age = input[i];

            if age.to_owned() == 0 {
                input[i] = 6;
                input.push(8);
            } else {
                input[i] = age - 1;
            }
        }
    }

    println!("{}", input.len());

    return input.len() as i32;
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
