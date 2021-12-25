use std::fmt::Display;

pub fn entry() {
    println!("Starting day 25!");

    let floor: Vec<Vec<char>> =
        aoc::read_input("./resources/y_2021/day_25_input.txt", move |line| {
            return line.chars().collect();
        });

    let mut sea_floor = SeaFloor::new(floor);
    println!("{}", sea_floor);

    let mut i = 0;
    while sea_floor.last_number_moves != 0 || i == 0 {
        i += 1;
        sea_floor.step();
        //println!("After step {}", i);
        //println!("{}", sea_floor);
    }

    println!("No moves after step {}", i);
}

struct SeaFloor {
    sea_floor: Vec<Vec<char>>,
    row_length: usize,
    col_length: usize,
    last_number_moves: u32,
}

impl SeaFloor {
    fn new(floor: Vec<Vec<char>>) -> SeaFloor {
        SeaFloor {
            row_length: floor[0].len(),
            col_length: floor.len(),
            sea_floor: floor,
            last_number_moves: 0,
        }
    }

    fn east_step(&mut self) -> u32 {
        let mut new_floor = self.sea_floor.clone();
        let mut number_moves = 0;
        for row in 0..self.col_length {
            for col in 0..self.row_length {
                if self.sea_floor[row][col] != '>' {
                    continue;
                }

                let mut col_plus_1 = col + 1;

                if col == self.row_length - 1 {
                    col_plus_1 = 0;
                }

                if self.sea_floor[row][col_plus_1] == '.' {
                    new_floor[row][col_plus_1] = '>';
                    new_floor[row][col] = '.';
                    number_moves += 1;
                }
            }
        }

        self.sea_floor = new_floor;
        number_moves
    }

    fn south_step(&mut self) -> u32 {
        let mut new_floor = self.sea_floor.clone();
        let mut number_moves = 0;
        for row in 0..self.col_length {
            for col in 0..self.row_length {
                if self.sea_floor[row][col] != 'v' {
                    continue;
                }

                let mut row_plus_1 = row + 1;

                if row == self.col_length - 1 {
                    row_plus_1 = 0;
                }

                if self.sea_floor[row_plus_1][col] == '.' {
                    new_floor[row_plus_1][col] = 'v';
                    new_floor[row][col] = '.';
                    number_moves += 1;
                }
            }
        }

        self.sea_floor = new_floor;
        number_moves
    }

    fn step(&mut self) {
        let mut number_moves = 0;
        number_moves += self.east_step();
        number_moves += self.south_step();
        self.last_number_moves = number_moves;
    }
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.sea_floor.iter() {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
