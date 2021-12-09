use super::input_helper;

#[derive(Debug)]
struct Board {
    drawn_numbers: Vec<i32>,
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
}

impl Board {
    fn is_marked(&self, number: &i32) -> bool {
        return self.drawn_numbers.contains(number);
    }

    fn check_number(&mut self, number: i32) -> Option<i32> {
        self.drawn_numbers.push(number);

        // Check for win
        let mut bingo = false;

        for row in &self.rows {
            if let true = row.iter().all(|n| self.is_marked(n)) {
                println!("Horizontal Match: {:?}", row);
                bingo = true;

                break;
            }
        }

        for col in &self.cols {
            if let true = col.iter().all(|n| self.is_marked(n)) {
                // println!("Vertical Match: {:?}", row);
                bingo = true;

                break;
            }
        }

        if bingo != true {
            return None;
        }

        let unmarked_rows: i32 = self
            .rows
            .iter()
            .flatten()
            .filter(|n| !self.is_marked(n))
            .sum();

        let unmarked_cols: i32 = self
            .cols
            .iter()
            .flatten()
            .filter(|n| !self.is_marked(n))
            .sum();

        let answer = (unmarked_rows + unmarked_cols) * number;

        return Some(answer / 2);
    }

    fn new(rows: Vec<Vec<i32>>) -> Self {
        let mut cols = vec![vec![0; 5]; 5];

        for (i, row) in rows.iter().enumerate() {
            for j in 0..5 {
                cols[j][i] = row[j];
            }
        }

        Board {
            drawn_numbers: vec![],
            rows,
            cols,
        }
    }
}

fn part_1() {
    let input = input_helper::get_input(4);

    let numbers: Vec<i32> = input
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let board_rows: Vec<String> = input[2..]
        .to_vec()
        .iter()
        .filter_map(|row| {
            if row != "" {
                Some(row.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut boards = Vec::new();

    for board in board_rows.windows(5).step_by(5) {
        let mut rows: Vec<Vec<i32>> = Vec::new();

        for row in board {
            let split_row: Vec<i32> = row
                .trim()
                .replace("  ", " ")
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();

            rows.push(split_row);
        }

        boards.push(Board::new(rows));
    }

    let mut answer = 0;

    'outer: for number in numbers {
        for board in boards.iter_mut() {
            if let Some(ans) = board.check_number(number) {
                answer = ans;

                break 'outer;
            }
        }
    }

    println!("Part 1: {:?}", answer);
}

pub fn main() {
    part_1()
}
