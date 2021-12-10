use std::fmt::Display;

use itertools::Itertools;

pub fn entry() {
    println!("Starting day 10!");

    let nav_subsystem_lines = aoc::read_input("./resources/y_2021/day_10_input.txt", move |line| {
        return line;
    });

    let mut line_parsers = vec![];
    for line in nav_subsystem_lines {
        line_parsers.push(NavSubsystemParser::new(&line));
    }

    let score = line_parsers
        .iter()
        .filter(|parser| !parser.valid)
        .map(|parser| parser.get_last_char())
        .fold(0, |score, char| match char {
            Some(NavSubsystemChar::Closing('(')) => score + 3,
            Some(NavSubsystemChar::Closing('[')) => score + 57,
            Some(NavSubsystemChar::Closing('{')) => score + 1197,
            Some(NavSubsystemChar::Closing('<')) => score + 25137,
            _ => panic!("Received unexpected last char for invalid line"),
        });

    println!("Part 1 score: {}", score);

    // Part 2
    let sorted_scores: Vec<u64> = line_parsers
        .iter_mut()
        .filter(|parser| parser.valid)
        .map(|parser| parser.complete())
        .map(|parser| parser.calc_score())
        .sorted()
        .collect();

    if sorted_scores.len() > 0 {
        let final_score = match sorted_scores.get((sorted_scores.len() - 1) / 2) {
            Some(score) => *score,
            None => 0,
        };
        println!("Part 2 score: {}", final_score);
    }
}

//Could probably add a lifetime here
#[derive(Debug)]
enum NavSubsystemChar {
    Opening(char),
    Closing(char),
}

impl NavSubsystemChar {
    fn new(char: &char) -> NavSubsystemChar {
        if *char == '(' {
            return NavSubsystemChar::Opening(char.clone());
        } else if *char == ')' {
            return NavSubsystemChar::Closing('(');
        } else if *char == '[' {
            return NavSubsystemChar::Opening(char.clone());
        } else if *char == ']' {
            return NavSubsystemChar::Closing('[');
        } else if *char == '{' {
            return NavSubsystemChar::Opening(char.clone());
        } else if *char == '}' {
            return NavSubsystemChar::Closing('{');
        } else if *char == '<' {
            return NavSubsystemChar::Opening(char.clone());
        } else if *char == '>' {
            return NavSubsystemChar::Closing('<');
        }

        panic!("Invalid char!");
    }

    fn is_matching_char(&self, other: &NavSubsystemChar) -> bool {
        match (self, other) {
            (Self::Opening(l0), Self::Closing(r0)) => l0 == r0,
            (Self::Closing(l0), Self::Opening(r0)) => l0 == r0,
            (_, _) => false,
        }
    }
}

impl PartialEq for NavSubsystemChar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Opening(l0), Self::Opening(r0)) => l0 == r0,
            (Self::Closing(l0), Self::Closing(r0)) => l0 == r0,
            (_, _) => false,
        }
    }
}

struct NavSubsystemParser {
    initial_chunks: Vec<NavSubsystemChar>,
    opening_chars: Vec<NavSubsystemChar>,
    completion_chars: Vec<NavSubsystemChar>,
    valid: bool,
}

impl NavSubsystemParser {
    fn new(line: &String) -> NavSubsystemParser {
        let mut chunks = vec![];
        let mut opening_chars: Vec<NavSubsystemChar> = vec![];
        let mut valid = true;

        for char in line.chars() {
            let nav_sub_char = NavSubsystemChar::new(&char);

            match nav_sub_char {
                NavSubsystemChar::Closing(c) => {
                    if opening_chars.is_empty() {
                        //println!("Line started with an opening char. Invalid!");
                        valid = false;
                        chunks.push(nav_sub_char);
                        break;
                    } else if !nav_sub_char
                        .is_matching_char(&opening_chars[opening_chars.len() - 1])
                    {
                        println!("Tried to close a char with an invalid closing char. Last opened char: {:?}, closing char: {:?}, {}", opening_chars[opening_chars.len() - 1], nav_sub_char, c);
                        valid = false;
                        chunks.push(nav_sub_char);
                        break;
                    } else {
                        //println!("Received a valid closing char. Last opened char: {:?}, closing char: {:?}", opening_chars[opening_chars.len() - 1], nav_sub_char);
                        chunks.push(nav_sub_char);
                        opening_chars.pop();
                    }
                }
                NavSubsystemChar::Opening(_) => {
                    //println!("Found an opening char {}. Adding to the vectors", char);
                    opening_chars.push(NavSubsystemChar::new(&char));
                    chunks.push(NavSubsystemChar::new(&char));
                }
            };
        }

        NavSubsystemParser {
            initial_chunks: chunks,
            opening_chars,
            completion_chars: vec![],
            valid,
        }
    }

    fn is_complete(&self) -> bool {
        self.opening_chars.is_empty()
    }

    fn get_last_char(&self) -> Option<&NavSubsystemChar> {
        self.initial_chunks.last()
    }

    fn complete(&mut self) -> &NavSubsystemParser {
        while !self.is_complete() {
            let last_opened = self.opening_chars.pop();

            match last_opened {
                Some(NavSubsystemChar::Opening('(')) => {
                    self.completion_chars.push(NavSubsystemChar::Closing('('))
                }
                Some(NavSubsystemChar::Opening('[')) => {
                    self.completion_chars.push(NavSubsystemChar::Closing('['))
                }
                Some(NavSubsystemChar::Opening('{')) => {
                    self.completion_chars.push(NavSubsystemChar::Closing('{'))
                }
                Some(NavSubsystemChar::Opening('<')) => {
                    self.completion_chars.push(NavSubsystemChar::Closing('<'))
                }
                _ => panic!("Invalid last opened character"),
            }
        }

        self
    }

    fn calc_score(&self) -> u64 {
        let mut score: u64 = 0;
        for completion_char in self.completion_chars.iter() {
            score = match completion_char {
                NavSubsystemChar::Closing('(') => score * 5 + 1,
                NavSubsystemChar::Closing('[') => score * 5 + 2,
                NavSubsystemChar::Closing('{') => score * 5 + 3,
                NavSubsystemChar::Closing('<') => score * 5 + 4,
                _ => panic!("Invalid completion char!"),
            };
        }

        score
    }
}

impl Display for NavSubsystemParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Valid: {}, complete: {}, closing chars: {:?}, chunks: {:?}",
            self.valid,
            self.is_complete(),
            self.opening_chars,
            self.initial_chunks
        )
    }
}
