use std::{collections::HashMap, hash::Hash};

pub fn entry() {
    println!("Starting day 20!");

    let mut board = DiracGameBoard::new(8, 4, 1000);
    let mut dice = DeterministicDice::new(100);

    let mut i = 0;
    while !board.done {
        if i % 2 == 0 {
            board.player_1_turn(&mut dice);
        } else {
            board.player_2_turn(&mut dice);
        }

        i += 1;
    }

    println!("Dice rolled {} times", dice.number_of_rolls);
    println!(
        "Player 1 has {} points, player 2 has {} points",
        board.player_1_score, board.player_2_score
    );

    let board = DiracGameBoard::new(8, 4, 21);
    let dice_rolls = vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];
    let mut multiverse_boards: HashMap<DiracGameBoard, u128> = HashMap::new();
    multiverse_boards.insert(board, 1);

    let mut i = 0;
    while !multiverse_boards.iter().all(|board| board.0.done) {
        let mut new_multiverse_boards: HashMap<DiracGameBoard, u128> = HashMap::new();

        if i % 2 == 0 {
            for entry in multiverse_boards.iter() {
                if !entry.0.done {
                    for dice_roll in dice_rolls.iter() {
                        let mut new_board = entry.0.clone();
                        new_board.player_1_advance_by(dice_roll.1);

                        let new_board_entry = new_multiverse_boards.entry(new_board).or_insert(0);
                        *new_board_entry += entry.1 * dice_roll.0 as u128;
                    }
                } else {
                    let new_board = entry.0.clone();

                    let new_board_entry = new_multiverse_boards.entry(new_board).or_insert(0);
                    *new_board_entry += entry.1;
                }
            }

            multiverse_boards = new_multiverse_boards;
        } else {
            for entry in multiverse_boards.iter() {
                if !entry.0.done {
                    for dice_roll in dice_rolls.iter() {
                        let mut new_board = entry.0.clone();
                        new_board.player_2_advance_by(dice_roll.1);

                        let new_board_entry = new_multiverse_boards.entry(new_board).or_insert(0);
                        *new_board_entry += entry.1 * dice_roll.0 as u128;
                    }
                } else {
                    let new_board = entry.0.clone();

                    let new_board_entry = new_multiverse_boards.entry(new_board).or_insert(0);
                    *new_board_entry += entry.1;
                }
            }

            multiverse_boards = new_multiverse_boards;
        }

        i += 1;
    }

    let mut won_uni_per_player: HashMap<u32, u128> = HashMap::new();
    won_uni_per_player.insert(
        1,
        multiverse_boards
            .iter()
            .filter(|b| b.0.has_player_1_won())
            .map(|b| b.1)
            .sum(),
    );
    won_uni_per_player.insert(
        2,
        multiverse_boards
            .iter()
            .filter(|b| b.0.has_player_2_won())
            .map(|b| b.1)
            .sum(),
    );

    println!("{:?}", won_uni_per_player);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct DiracGameBoard {
    player_1_score: u64,
    player_2_score: u64,
    player_1_position: u32,
    player_2_position: u32,
    done: bool,
    winning_score: u64,
}

impl DiracGameBoard {
    fn new(
        player_1_starting_position: u32,
        player_2_starting_position: u32,
        winning_score: u64,
    ) -> DiracGameBoard {
        if player_1_starting_position < 1 || player_1_starting_position > 10 {
            panic!("Invalid starting position. Must be within 1 and 10");
        }

        if player_2_starting_position < 1 || player_2_starting_position > 10 {
            panic!("Invalid starting position. Must be within 1 and 10");
        }

        DiracGameBoard {
            player_1_position: player_1_starting_position,
            player_2_position: player_2_starting_position,
            player_1_score: 0,
            player_2_score: 0,
            done: false,
            winning_score,
        }
    }

    fn has_player_1_won(&self) -> bool {
        self.player_1_score >= self.winning_score
    }

    fn has_player_2_won(&self) -> bool {
        self.player_2_score >= self.winning_score
    }

    fn player_1_turn(&mut self, dice: &mut impl DiceRoll) {
        if self.done {
            println!("Game is done already! Player 1 has {} points and player 2 has {} points. Create a new board to start a new game!", self.player_1_score, self.player_2_score);
            return;
        }
        let mut roll = 0;
        for _ in 0..3 {
            roll += dice.roll();
        }

        self.player_1_advance_by(roll);
    }

    fn player_1_advance_by(&mut self, roll: u32) {
        if self.done {
            //println!("Game is done already! Player 1 has {} points and player 2 has {} points. Create a new board to start a new game!", self.player_1_score, self.player_2_score);
            return;
        }

        self.player_1_position = self.player_1_position + roll;
        if self.player_1_position > 10 {
            self.player_1_position = self.player_1_position % 10;
        }
        if self.player_1_position == 0 {
            self.player_1_position = 10;
        }

        self.player_1_score += self.player_1_position as u64;

        if self.has_player_1_won() {
            /*println!(
                "Player 1 is now on tile {} and has won with {} points!",
                self.player_1_position, self.player_1_score
            );*/
            self.done = true;
        } else {
            /*println!(
                "Player 1 is now on tile {} and has {} points!",
                self.player_1_position, self.player_1_score
            );*/
        }
    }

    fn player_2_turn(&mut self, dice: &mut impl DiceRoll) {
        if self.done {
            println!("Game is done already! Player 1 has {} points and player 2 has {} points. Create a new board to start a new game!", self.player_1_score, self.player_2_score);
            return;
        }
        let mut roll = 0;
        for _ in 0..3 {
            roll += dice.roll();
        }

        self.player_2_advance_by(roll);
    }

    fn player_2_advance_by(&mut self, roll: u32) {
        if self.done {
            //println!("Game is done already! Player 1 has {} points and player 2 has {} points. Create a new board to start a new game!", self.player_1_score, self.player_2_score);
            return;
        }

        self.player_2_position = self.player_2_position + roll;
        if self.player_2_position > 10 {
            self.player_2_position = self.player_2_position % 10;
        }
        if self.player_2_position == 0 {
            self.player_2_position = 10;
        }

        self.player_2_score += self.player_2_position as u64;

        if self.has_player_2_won() {
            /*println!(
                "Player 2 is now on tile {} and has won with {} points!",
                self.player_2_position, self.player_2_score
            );*/
            self.done = true;
        } else {
            /*println!(
                "Player 2 is now on tile {} and has {} points!",
                self.player_2_position, self.player_2_score
            );*/
        }
    }
}

trait DiceRoll {
    fn roll(&mut self) -> u32;
}

struct DeterministicDice {
    max_roll: u32,
    last_roll: u32,
    number_of_rolls: u32,
}

impl DeterministicDice {
    fn new(max_roll: u32) -> DeterministicDice {
        DeterministicDice {
            max_roll,
            last_roll: 0,
            number_of_rolls: 0,
        }
    }
}

impl DiceRoll for DeterministicDice {
    fn roll(&mut self) -> u32 {
        if self.last_roll >= self.max_roll {
            //println!("Dice reached 100 last roll! Starting back at 1");
            self.last_roll = 0;
        }

        self.last_roll += 1;
        //println!("Rolled {}!", self.last_roll);
        self.number_of_rolls += 1;
        self.last_roll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_by() {
        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(1);
        board.player_2_advance_by(1);

        assert_eq!(board.player_1_position, 2);
        assert_eq!(board.player_2_position, 2);

        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(5);
        board.player_2_advance_by(5);

        assert_eq!(board.player_1_position, 6);
        assert_eq!(board.player_2_position, 6);

        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(9);
        board.player_2_advance_by(9);

        assert_eq!(board.player_1_position, 10);
        assert_eq!(board.player_2_position, 10);

        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(10);
        board.player_2_advance_by(10);

        assert_eq!(board.player_1_position, 1);
        assert_eq!(board.player_2_position, 1);

        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(19);
        board.player_2_advance_by(19);

        assert_eq!(board.player_1_position, 10);
        assert_eq!(board.player_2_position, 10);

        let mut board = DiracGameBoard::new(1, 1, 50);
        board.player_1_advance_by(25);
        board.player_2_advance_by(25);

        assert_eq!(board.player_1_position, 6);
        assert_eq!(board.player_2_position, 6);
    }
}
