use super::input_helper;

enum Commonality {
    Most,
    Least,
}

fn part_2() -> i32 {
    let input = input_helper::get_input(3);

    fn most_or_least_common(index: usize, vals: &Vec<String>, commonality: Commonality) -> i32 {
        let mut ones = Vec::new();
        let mut zeros = Vec::new();

        if vals.len() == 1 {
            let last_remaining = i32::from_str_radix(vals[0].as_str(), 2).unwrap();
            return last_remaining;
        }

        for binary in vals {
            let bit_chars: Vec<char> = binary.chars().collect();
            println!("index: {}", index);

            match bit_chars[index] {
                '1' => ones.push(binary.to_owned()),

                '0' => zeros.push(binary.to_owned()),
                _ => {}
            }
        }

        match commonality {
            Commonality::Most => {
                if ones.len() >= zeros.len() {
                    most_or_least_common(index + 1, &ones, commonality)
                } else {
                    most_or_least_common(index + 1, &zeros, commonality)
                }
            }
            Commonality::Least => {
                if zeros.len() <= ones.len() {
                    most_or_least_common(index + 1, &zeros, commonality)
                } else {
                    most_or_least_common(index + 1, &ones, commonality)
                }
            }
        }
    }

    let oxygen = most_or_least_common(0, &input, Commonality::Most);
    let co2 = most_or_least_common(0, &input, Commonality::Least);
    let answer = oxygen * co2;

    println!("Oxygen: {}", oxygen);
    println!("CO2: {}", co2);
    println!("Multiplied: {}", answer);

    return answer;
}

pub fn main() {
    part_2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_2() {
        assert_eq!(super::part_2(), 587895)
    }
}
