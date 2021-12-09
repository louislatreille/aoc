use std::{fmt::Display, io::ErrorKind};

pub fn entry() {
    println!("Starting day 8!");

    let note_lines = aoc::read_input("./resources/y_2021/day_8_input.txt", move |line| {
        return parse_note_line(line);
    });

    let (_, digits) = agg_note_lines(&note_lines);

    // Part 1
    println!("Found {} simple digits", count_simple_digits(digits));
    println!("");

    // Part 2
    let mut total = 0;
    for (uniq_segs, digits) in note_lines {
        let seven_seg = SevenSegmentsDecoder::new(&uniq_segs);

        println!("{}", seven_seg);

        let number = match seven_seg.decode_digits(&digits) {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error with input: {:?} | {:?}", uniq_segs, digits);
                panic!();
            }
        };

        total += number;
    }

    println!("Total is: {}", total);
}

fn parse_note_line(line: String) -> (Vec<String>, Vec<String>) {
    let mut pipe_splits = line.split(" | ");

    // nth consumes all elements up to and including the index
    let uniq_segs = match pipe_splits.nth(0) {
        Some(val) => val.split(" ").map(|s| s.to_owned()),
        None => panic!("Invalid input!"),
    };

    let digits = match pipe_splits.nth(0) {
        Some(val) => val.split(" ").map(|s| s.to_owned()),
        None => panic!("Invalid input!"),
    };

    let uniq_seg = uniq_segs.collect();
    let digits = digits.collect();

    println!("{:?} | {:?}", uniq_seg, digits);

    (uniq_seg, digits)
}

fn agg_note_lines(note_lines: &Vec<(Vec<String>, Vec<String>)>) -> (Vec<String>, Vec<String>) {
    let mut uniq_segs = vec![];
    let mut digits = vec![];

    note_lines
        .iter()
        .for_each(|(current_segs, current_digits)| {
            uniq_segs.extend(current_segs.iter().map(|s| s.to_owned()));
            digits.extend(current_digits.iter().map(|s| s.to_owned()));
        });

    (uniq_segs, digits)
}

fn count_simple_digits(digits: Vec<String>) -> usize {
    let simple_digits_seg_counts = vec![2, 3, 4, 7];
    digits
        .iter()
        .filter(|digits| simple_digits_seg_counts.contains(&digits.len()))
        .count()
}

struct SevenSegmentsDecoder {
    top: char,
    top_left: char,
    top_right: char,
    mid: char,
    down_left: char,
    down_right: char,
    down: char,
    num_two_chars: Vec<char>,
    num_three_chars: Vec<char>,
    num_five_chars: Vec<char>,
    num_six_chars: Vec<char>,
    num_nine_chars: Vec<char>,
    num_zero_chars: Vec<char>,
}

impl SevenSegmentsDecoder {
    fn new(uniq_segs: &Vec<String>) -> SevenSegmentsDecoder {
        let num_one_segs = match uniq_segs.iter().filter(|seg| seg.len() == 2).nth(0) {
            Some(val) => val,
            None => panic!("Couldn't find a 1 in the unique segments"),
        };

        let num_seven_segs = match uniq_segs.iter().filter(|seg| seg.len() == 3).nth(0) {
            Some(val) => val,
            None => panic!("Couldn't find a 7 in the unique segments"),
        };

        let num_four_segs = match uniq_segs.iter().filter(|seg| seg.len() == 4).nth(0) {
            Some(val) => val,
            None => panic!("Couldn't find a 4 in the unique segments"),
        };

        let num_eight_segs = match uniq_segs.iter().filter(|seg| seg.len() == 7).nth(0) {
            Some(val) => val,
            None => panic!("Couldn't find a 8 in the unique segments"),
        };

        let six_segs: Vec<&String> = uniq_segs.iter().filter(|seg| seg.len() == 6).collect();
        let five_segs: Vec<&String> = uniq_segs.iter().filter(|seg| seg.len() == 5).collect();

        let top_segment = num_seven_segs
            .chars()
            .filter(|char| num_one_segs.chars().find(|c| c == char).is_none())
            .nth(0)
            .unwrap();

        //println!("Top segment: {}", top_segment);

        let down_right_segment = num_one_segs
            .chars()
            .filter(|char| {
                six_segs
                    .iter()
                    .all(|six_seg| six_seg.chars().find(|c| c == char).is_some())
            })
            .nth(0)
            .unwrap();

        //println!("Down right segment: {}", down_right_segment);

        let top_right_segment = num_one_segs
            .chars()
            .filter(|char| *char != down_right_segment)
            .nth(0)
            .unwrap();

        //println!("Top right segment: {}", top_right_segment);

        let mid_segment = num_four_segs
            .chars()
            .filter(|char| *char != top_right_segment && *char != down_right_segment)
            .filter(|char| {
                five_segs
                    .iter()
                    .all(|five_seg| five_seg.chars().find(|c| c == char).is_some())
            })
            .nth(0)
            .unwrap();

        //println!("Mid segment: {}", mid_segment);

        let top_left_segment = num_four_segs
            .chars()
            .filter(|char| {
                *char != mid_segment && *char != top_right_segment && *char != down_right_segment
            })
            .nth(0)
            .unwrap();

        //println!("Top left segment: {}", top_left_segment);

        let down_segment = num_eight_segs
            .chars()
            .filter(|char| {
                *char != top_right_segment
                    && *char != down_right_segment
                    && *char != mid_segment
                    && *char != top_left_segment
                    && *char != top_segment
            })
            .filter(|char| {
                six_segs
                    .iter()
                    .all(|six_seg| six_seg.chars().find(|c| c == char).is_some())
            })
            .nth(0)
            .unwrap();

        //println!("Down segment: {}", down_segment);

        let down_left_segment = num_eight_segs
            .chars()
            .filter(|char| {
                *char != mid_segment
                    && *char != top_right_segment
                    && *char != down_right_segment
                    && *char != down_segment
                    && *char != top_left_segment
                    && *char != top_segment
            })
            .nth(0)
            .unwrap();

        //println!("Down left segment: {}", down_left_segment);

        SevenSegmentsDecoder {
            top: top_segment,
            top_left: top_left_segment,
            top_right: top_right_segment,
            mid: mid_segment,
            down_left: down_left_segment,
            down_right: down_right_segment,
            down: down_segment,
            num_two_chars: vec![
                top_segment,
                top_right_segment,
                mid_segment,
                down_left_segment,
                down_segment,
            ],
            num_three_chars: vec![
                top_segment,
                top_right_segment,
                mid_segment,
                down_right_segment,
                down_segment,
            ],
            num_five_chars: vec![
                top_segment,
                top_left_segment,
                mid_segment,
                down_right_segment,
                down_segment,
            ],
            num_six_chars: vec![
                top_segment,
                top_left_segment,
                mid_segment,
                down_left_segment,
                down_right_segment,
                down_segment,
            ],
            num_nine_chars: vec![
                top_segment,
                top_left_segment,
                top_right_segment,
                mid_segment,
                down_right_segment,
                down_segment,
            ],
            num_zero_chars: vec![
                top_segment,
                top_left_segment,
                top_right_segment,
                down_left_segment,
                down_right_segment,
                down_segment,
            ],
        }
    }

    fn decode_digits(&self, digits: &Vec<String>) -> Result<u32, std::io::ErrorKind> {
        if digits.len() != 4 {
            panic!("Invalid input!");
        }

        let mut i = 4;
        digits.iter().try_fold(0, |sum, digit| {
            let num = match self.decode_single_digit(digit) {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("Failed to decode digits! {:?}", digits);
                    return Err(err);
                }
            };

            i -= 1;
            Ok(sum + num * 10_u32.pow(i))
        })
    }

    // TODO how to do that with Result?
    fn decode_single_digit(&self, digit: &String) -> Result<u32, std::io::ErrorKind> {
        match digit.len() {
            0 => {
                eprintln!("Input contains no char");
                Err(ErrorKind::InvalidInput)
            }
            1 => {
                eprintln!(
                    "The input contains only one char. Impossible to create a number! {}",
                    digit
                );
                Err(ErrorKind::InvalidInput)
            }
            2 => return Ok(1),
            3 => return Ok(7),
            4 => return Ok(4),
            5 => {
                if self.is_two(digit) {
                    return Ok(2);
                }

                if self.is_three(digit) {
                    return Ok(3);
                }

                if self.is_five(digit) {
                    return Ok(5);
                }

                eprintln!(
                    "Didn't find any matching number with the five segments received!: {}",
                    digit
                );

                Err(ErrorKind::InvalidInput)
            }
            6 => {
                if self.is_six(digit) {
                    return Ok(6);
                }

                if self.is_nine(digit) {
                    return Ok(9);
                }

                if self.is_zero(digit) {
                    return Ok(0);
                }

                eprintln!(
                    "Didn't find any matching number with the six segments received!: {}",
                    digit
                );

                Err(ErrorKind::InvalidInput)
            }
            7 => return Ok(8),
            _ => {
                eprintln!(
                    "Received more than 7 chars. Impossible to determine number! {}",
                    digit
                );
                Err(ErrorKind::InvalidInput)
            }
        }
    }

    fn is_two(&self, digit: &String) -> bool {
        self.num_two_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }

    fn is_three(&self, digit: &String) -> bool {
        self.num_three_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }

    fn is_five(&self, digit: &String) -> bool {
        self.num_five_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }

    fn is_six(&self, digit: &String) -> bool {
        self.num_six_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }

    fn is_nine(&self, digit: &String) -> bool {
        self.num_nine_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }

    fn is_zero(&self, digit: &String) -> bool {
        self.num_zero_chars
            .iter()
            .all(|char| digit.chars().find(|c| c == char).is_some())
    }
}

impl Display for SevenSegmentsDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " {}{}{} ", self.top, self.top, self.top)?;
        writeln!(f, "{}   {}", self.top_left, self.top_right)?;
        writeln!(f, "{}   {}", self.top_left, self.top_right)?;
        writeln!(f, " {}{}{} ", self.mid, self.mid, self.mid)?;
        writeln!(f, "{}   {}", self.down_left, self.down_right)?;
        writeln!(f, "{}   {}", self.down_left, self.down_right)?;
        writeln!(f, " {}{}{} ", self.down, self.down, self.down)?;

        Ok(())
    }
}
