use std::fmt::Display;

pub fn entry() {
    println!("Starting day 4!");

    let inputs = aoc::read_input("./resources/y_2021/day_4_input.txt", move |line| {
        return line;
    });

    let (drawn_numbers, mut bingo_boards) = create_game(&inputs);

    println!("Drawn numbers: {:?}", drawn_numbers);

    for board in &bingo_boards {
        println!("{}", board);
    }

    // Part 1
    for drawn_number in drawn_numbers {
        match draw_number_and_check_winner(drawn_number, &mut bingo_boards) {
            Some(board) => {
                println!(
                    "We have a winner! You're score is {}",
                    drawn_number * board.calculate_score()
                );
                break;
            }
            None => println!("No winner yet!"),
        }
    }

    let (drawn_numbers, mut bingo_boards) = create_game(&inputs);

    // Part 2
    let mut last_winning_board_score = None;
    let mut last_winning_number = None;
    for drawn_number in drawn_numbers {
        draw_number(drawn_number, &mut bingo_boards);

        bingo_boards.retain(|board| {
            let winner = board.is_won();

            if winner {
                let score = board.calculate_score();
                println!("Winning board, but we don't want it! Keep going...");
                println!("Score was {} and number was {}", score, drawn_number);

                last_winning_board_score = Some(score);
                last_winning_number = Some(drawn_number);
            }

            !winner
        });
    }

    if last_winning_board_score.is_some() && last_winning_number.is_some() {
        println!(
            "Last board to win has score: {}",
            last_winning_board_score.unwrap() * last_winning_number.unwrap()
        );
    } else {
        panic!("No winning board?!");
    }
}

fn create_game(lines: &Vec<String>) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut boards = vec![];

    let numbers = match lines.get(0) {
        Some(line) => line.split(","),
        None => panic!("Invalid input"),
    };

    let drawn_numbers: Vec<u32> = numbers.map(|num| num.parse::<u32>().unwrap()).collect();

    let mut current_board = BingoBoard::init();
    for (index, line) in lines.iter().enumerate() {
        if index < 2 {
            continue;
        }

        if line.is_empty() {
            boards.push(current_board);
            current_board = BingoBoard::init();
            continue;
        }

        current_board.add_row(line);
    }

    boards.push(current_board);

    (drawn_numbers, boards)
}

fn draw_number_and_check_winner(number: u32, boards: &mut Vec<BingoBoard>) -> Option<&BingoBoard> {
    for board in boards.iter_mut() {
        board.mark_number(number);

        if board.is_won() {
            return Some(board);
        }
    }

    None
}

fn draw_number(number: u32, boards: &mut Vec<BingoBoard>) {
    for board in boards.iter_mut() {
        board.mark_number(number);
    }
}

struct BingoBoard {
    tiles: Vec<Vec<(u32, bool)>>,
}

impl BingoBoard {
    fn new(board: Vec<Vec<(u32, bool)>>) -> BingoBoard {
        BingoBoard { tiles: board }
    }

    fn init() -> BingoBoard {
        BingoBoard { tiles: vec![] }
    }

    fn add_row<T: AsRef<str>>(&mut self, line: T) {
        let numbers = line.as_ref().split(" ");
        let numbers: Vec<(u32, bool)> = numbers
            .filter(|str| !str.is_empty())
            .map(|str| (str.parse().unwrap(), false))
            .collect();

        self.tiles.push(numbers);
    }

    fn mark_number(&mut self, number: u32) {
        self.tiles
            .iter_mut()
            .flat_map(|tile| tile)
            .for_each(|tile| {
                if tile.0 == number {
                    tile.1 = true;
                }
            });
    }

    fn is_won(&self) -> bool {
        self.is_won_on_rows() || self.is_won_on_columns()
    }

    fn is_won_on_rows(&self) -> bool {
        self.tiles
            .iter()
            .any(|tiles_row| tiles_row.iter().all(|tile| tile.1))
    }

    fn is_won_on_columns(&self) -> bool {
        let size = self.tiles.get(0).unwrap().len();

        for i in 0..size {
            if self
                .tiles
                .iter()
                .all(|tiles_row| tiles_row.get(i).unwrap().1)
            {
                return true;
            }
        }

        return false;
    }

    fn calculate_score(&self) -> u32 {
        self.tiles
            .iter()
            .flat_map(|tile| tile)
            .filter(|tile| tile.1 == false)
            .map(|tile| tile.0)
            .sum()
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_won_return_true_when_won_on_rows() {
        let board = BingoBoard::new(vec![
            vec![(22, true), (13, true), (17, true), (11, true), (0, true)],
            vec![(8, false), (2, false), (23, false), (4, false), (24, false)],
            vec![
                (21, false),
                (9, false),
                (14, false),
                (16, false),
                (7, false),
            ],
            vec![(6, false), (10, false), (3, false), (18, false), (5, false)],
            vec![
                (1, false),
                (12, false),
                (20, false),
                (15, false),
                (19, false),
            ],
        ]);

        assert_eq!(true, board.is_won());
    }

    #[test]
    fn test_is_won_return_true_when_won_on_columns() {
        let board = BingoBoard::new(vec![
            vec![(22, true), (13, true), (17, false), (11, true), (0, true)],
            vec![(8, true), (2, false), (23, false), (4, false), (24, false)],
            vec![(21, true), (9, false), (14, false), (16, false), (7, false)],
            vec![(6, true), (10, false), (3, false), (18, false), (5, false)],
            vec![
                (1, true),
                (12, false),
                (20, false),
                (15, false),
                (19, false),
            ],
        ]);

        assert_eq!(true, board.is_won());
    }

    #[test]
    fn test_is_won_return_false_when_not_won() {
        let board = BingoBoard::new(vec![
            vec![(22, true), (13, true), (17, false), (11, true), (0, true)],
            vec![(8, true), (2, false), (23, false), (4, false), (24, false)],
            vec![(21, true), (9, false), (14, false), (16, false), (7, false)],
            vec![(6, false), (10, false), (3, true), (18, false), (5, false)],
            vec![(1, true), (12, false), (20, false), (15, false), (19, true)],
        ]);

        assert_eq!(false, board.is_won());
    }

    #[test]
    fn test_mark_number_works_as_expected() {
        let mut board = BingoBoard::new(vec![
            vec![(22, true), (13, true), (17, false), (11, true), (0, true)],
            vec![(8, true), (2, false), (23, false), (4, false), (24, false)],
            vec![(21, true), (9, false), (14, false), (16, false), (7, false)],
            vec![(6, false), (10, false), (3, true), (18, false), (5, false)],
            vec![(1, true), (12, false), (20, false), (15, false), (19, true)],
        ]);

        assert_eq!(false, board.is_won());

        board.mark_number(6);

        assert_eq!(true, board.is_won());
    }

    #[test]
    fn test_calc_score_works_as_expected() {
        let board = BingoBoard::new(vec![
            vec![(22, true), (13, true), (17, false), (11, true), (0, true)],
            vec![(8, true), (2, false), (23, false), (4, false), (24, false)],
            vec![(21, true), (9, false), (14, false), (16, false), (7, true)],
            vec![(6, true), (10, false), (3, true), (18, false), (5, false)],
            vec![
                (1, true),
                (12, false),
                (20, false),
                (15, false),
                (19, false),
            ],
        ]);

        assert_eq!(208, board.calculate_score());
    }
}
