use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn get_input(day: i32) -> Vec<String> {
    let file = File::open(format!("inputs/day{:02}_input.txt", day)).expect("Input file not found");
    let buffer = BufReader::new(file);

    let lines: Vec<String> = buffer.lines().collect::<Result<_, _>>().unwrap();

    return lines;
}

pub trait StringVecExt {
    fn to_i32(&mut self) -> Vec<i32>;
}

impl StringVecExt for Vec<String> {
    fn to_i32(&mut self) -> Vec<i32> {
        self.iter()
            .map(|s| {
                s.parse()
                    .expect("Can't convert string vector to i32 vector")
            })
            .collect()
    }
}
