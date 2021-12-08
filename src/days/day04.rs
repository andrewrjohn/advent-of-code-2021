struct Board {
    drawn_numbers: Vec<i32>,
    grid: [[i32; 5]; 5],
}

impl Board {
    fn add_number(&mut self, number: i32) {
        self.drawn_numbers.push(number);

        // Check for win
        let mut columns: [[i32; 5]; 5] = [[0; 5]; 5];
        let mut bingo = false;

        for (i, row) in self.grid.iter().enumerate() {
            // Horizontal row
            if let true = row.iter().all(|n| self.drawn_numbers.contains(n)) {
                println!("Horizontal Match: {:?}", row);

                bingo = true;
            }

            for (j, num) in row.iter().enumerate() {
                columns[j][i] = num.to_owned();
            }
        }

        // println!("{:?}", columns);
        // println!("----");
        for row in columns {
            if let true = row.iter().all(|n| self.drawn_numbers.contains(n)) {
                println!("Vertical Match: {:?}", row);
                bingo = true
            }
        }

        if bingo != true {
            return;
        }
        let mut unmarked_rows_sum = 0;
        let mut unmarked_cols_sum = 0;

        for row in self.grid {
            unmarked_rows_sum += row
                .iter()
                .filter(|n| self.drawn_numbers.contains(n) == false)
                .sum::<i32>();
        }

        for col in columns {
            unmarked_cols_sum += col
                .iter()
                .filter(|n| self.drawn_numbers.contains(n) == false)
                .sum::<i32>();
        }

        let answer = (unmarked_cols_sum + unmarked_rows_sum) * number;

        println!("Answer: {}", answer)
    }

    fn new(grid: [[i32; 5]; 5]) -> Self {
        Board {
            drawn_numbers: vec![],
            grid,
        }
    }
}

fn part_1() {
    let mut board = Board::new([
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 1],
        [2, 3, 4, 5, 6],
        [7, 8, 9, 1, 2],
        [3, 4, 5, 6, 7],
    ]);

    // Horizontal match
    // board.add_number(1);
    // board.add_number(2);
    // board.add_number(3);
    // board.add_number(4);
    // board.add_number(5);

    // Vertical match
    board.add_number(1);
    board.add_number(6);
    board.add_number(2);
    board.add_number(7);
    board.add_number(3);
}

pub fn main() {
    part_1()
}
