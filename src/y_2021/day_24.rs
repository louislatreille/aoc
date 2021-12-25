use std::collections::{HashMap, VecDeque};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::SystemTime;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn entry() {
    println!("Starting day 24!");

    // Process all digits
    let mut z_ranges = HashMap::new();
    z_ranges.insert(14, 1..=26 * 2);
    z_ranges.insert(13, 1..=600);
    z_ranges.insert(12, 1..=15000);
    z_ranges.insert(11, 1..=400000);
    z_ranges.insert(10, 0..=15000);
    z_ranges.insert(9, 0..=400000);
    z_ranges.insert(8, 0..=10000000);
    z_ranges.insert(7, 0..=400000);
    z_ranges.insert(6, 0..=15000);
    z_ranges.insert(5, 0..=600);
    z_ranges.insert(4, 0..=15000);
    z_ranges.insert(3, 0..=600);
    z_ranges.insert(2, 0..=26 * 2);
    z_ranges.insert(1, 0..=0);

    let mut w_values: HashMap<i64, Vec<Vec<u8>>> = HashMap::new();
    w_values.insert(0, vec![vec![]]);

    for i in (1..=14).rev() {
        let mut file_name = "./resources/y_2021/day_24_input_".to_string();
        file_name.push_str(&i.to_string());
        file_name.push_str(".txt");

        println!("Now at digit {}, reading file {}", i, file_name);
        let start = SystemTime::now();

        let instructions = aoc::read_input(&file_name, move |line| {
            return line;
        });
        let mut new_w_values: HashMap<i64, Vec<Vec<u8>>> = HashMap::new();
        let mut alu = Alu::init();

        for z in z_ranges.get(&i).unwrap().clone() {
            for w in 1..=9 {
                alu.clear();
                alu.processing_units.insert('z', z);
                alu.input_queue = vec![w].iter().rev().map(|v| v.clone()).collect();
                for instr in instructions.iter() {
                    alu.exec_instruction(instr);
                }

                let potentital_z = alu.get_val(&'z');
                let z_target = w_values.get(&potentital_z);
                if z_target.is_some() {
                    let inputs = z_target.unwrap();

                    let mut inputs = inputs.clone();
                    for input in inputs.iter_mut() {
                        input.push(w as u8);
                    }

                    let entry = new_w_values.entry(z).or_insert(vec![]);
                    entry.extend(inputs);

                    //break; maybe we don't need that?
                }
            }
        }

        w_values = new_w_values;
        let z_extremes = match w_values.keys().minmax() {
            itertools::MinMaxResult::NoElements => (&0, &0),
            itertools::MinMaxResult::OneElement(min_max) => (min_max, min_max),
            itertools::MinMaxResult::MinMax(min, max) => (min, max),
        };
        println!(
            "Finished digit {}. Took {}ms. Found {} values! Max: {}, min: {}",
            i,
            SystemTime::now().duration_since(start).unwrap().as_millis(),
            w_values.len(),
            z_extremes.1,
            z_extremes.0
        );
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("possible_inputs.txt")
        .unwrap();

    for values in w_values {
        if let Err(e) = writeln!(file, "{:?}", values) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    /*let instructions = aoc::read_input("./resources/y_2021/day_24_input.txt", move |line| {
        return line;
    });

    let inputs: Vec<i64> = vec![5, 9, 9, 9, 8, 4, 9, 4, 9, 3, 9, 7, 2, 9]
        .iter()
        .rev()
        .map(|i| i.clone())
        .collect();
    let mut alu = Alu::init_with_inputs(inputs);
    //alu.processing_units.insert('z', 9158);

    for instr in instructions.iter() {
        alu.exec_instruction(instr);
    }

    println!("z value is {}", alu.get_val(&'z'));*/
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"[a-z]+").unwrap();
}

struct Alu {
    processing_units: HashMap<char, i64>,
    input_queue: VecDeque<i64>,
}

impl Alu {
    fn init() -> Alu {
        Alu {
            processing_units: HashMap::new(),
            input_queue: VecDeque::new(),
        }
    }

    fn init_with_inputs(inputs: Vec<i64>) -> Alu {
        Alu {
            processing_units: HashMap::new(),
            input_queue: inputs.iter().rev().map(|v| v.clone()).collect(),
        }
    }

    fn clear(&mut self) {
        self.processing_units.clear();
    }

    fn exec_instruction<T: AsRef<str>>(&mut self, instr_str: T) {
        let split_instr: Vec<&str> = instr_str.as_ref().split(" ").collect();
        let instr = split_instr[0];
        let pu_1 = split_instr[1].chars().nth(0).unwrap();

        if instr == "inp" {
            let pu_value = self.processing_units.entry(pu_1).or_insert(0);
            *pu_value = match self.input_queue.pop_back() {
                Some(val) => val,
                None => {
                    eprintln!("Tried read from an empty input queue!");
                    panic!();
                }
            }
        } else {
            let val = match RE.is_match(split_instr[2]) {
                false => split_instr[2].parse::<i64>().unwrap(),
                true => self
                    .processing_units
                    .entry(split_instr[2].chars().nth(0).unwrap())
                    .or_insert(0)
                    .clone(),
            };

            match instr {
                "add" => {
                    let pu_value = self.processing_units.entry(pu_1).or_insert(0);
                    *pu_value += val;
                }
                "mul" => {
                    let pu_value = self.processing_units.entry(pu_1).or_insert(0);
                    *pu_value *= val;
                }
                "div" => {
                    if val == 0 {
                        eprintln!("Tried to divide by 0!");
                        panic!();
                    }

                    let pu_value = self.processing_units.entry(pu_1).or_insert(0);
                    *pu_value /= val;
                }
                "mod" => {
                    if val <= 0 {
                        eprintln!("Tried to modulo by {}!", val);
                        panic!();
                    }

                    let pu_value = self.processing_units.entry(pu_1).or_insert(0);

                    if *pu_value < 0 {
                        eprintln!("Tried to modulo a {}!", pu_value);
                        panic!();
                    }

                    *pu_value %= val;
                }
                "eql" => {
                    let pu_value = self.processing_units.entry(pu_1).or_insert(0);

                    if *pu_value == val {
                        *pu_value = 1;
                    } else {
                        *pu_value = 0;
                    }
                }
                _ => {
                    eprintln!("Unknown instruction {}!", instr);
                    panic!();
                }
            }
        }
    }

    fn get_val(&self, processing_unit: &char) -> i64 {
        self.processing_units
            .get(processing_unit)
            .unwrap_or(&0)
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_queue() {
        let mut alu = Alu::init_with_inputs(vec![5, -1, 10, 7]);

        alu.exec_instruction("inp a");
        assert_eq!(5, alu.get_val(&'a'));

        alu.exec_instruction("inp b");
        assert_eq!(-1, alu.get_val(&'b'));

        alu.exec_instruction("inp c");
        assert_eq!(10, alu.get_val(&'c'));

        alu.exec_instruction("inp w");
        assert_eq!(7, alu.get_val(&'w'));
    }

    #[test]
    fn test_alu_add() {
        let mut alu = Alu::init();

        alu.exec_instruction("add a 4");
        assert_eq!(4, alu.get_val(&'a'));

        alu.exec_instruction("add b 8");
        assert_eq!(8, alu.get_val(&'b'));

        alu.exec_instruction("add a b");
        assert_eq!(12, alu.get_val(&'a'));
        assert_eq!(8, alu.get_val(&'b'));

        alu.exec_instruction("add c -5");
        assert_eq!(-5, alu.get_val(&'c'));
    }

    #[test]
    fn test_alu_mul() {
        let mut alu = Alu::init_with_inputs(vec![5]);
        alu.exec_instruction("inp a");

        alu.exec_instruction("mul a 4");
        assert_eq!(20, alu.get_val(&'a'));

        alu.exec_instruction("mul a -5");
        assert_eq!(-100, alu.get_val(&'a'));

        alu.exec_instruction("mul b a");
        assert_eq!(-100, alu.get_val(&'a'));
        assert_eq!(0, alu.get_val(&'b'));
    }

    #[test]
    fn test_alu_div() {
        let mut alu = Alu::init_with_inputs(vec![5, 6, -4, -8, -9]);

        alu.exec_instruction("inp a");
        alu.exec_instruction("div a 5");
        assert_eq!(1, alu.get_val(&'a'));

        alu.exec_instruction("inp b");
        alu.exec_instruction("div b 5");
        assert_eq!(1, alu.get_val(&'b'));

        alu.exec_instruction("inp c");
        alu.exec_instruction("div c 4");
        assert_eq!(-1, alu.get_val(&'c'));

        alu.exec_instruction("inp d");
        alu.exec_instruction("div d -8");
        assert_eq!(1, alu.get_val(&'d'));

        alu.exec_instruction("inp e");
        alu.exec_instruction("div e 5");
        assert_eq!(-1, alu.get_val(&'e'));

        alu.exec_instruction("div a b");
        assert_eq!(1, alu.get_val(&'a'));
    }

    #[test]
    fn test_alu_mod() {
        let mut alu = Alu::init_with_inputs(vec![5, 6, -4, -8, -9]);

        alu.exec_instruction("inp a");
        alu.exec_instruction("mod a 5");
        assert_eq!(0, alu.get_val(&'a'));

        alu.exec_instruction("inp b");
        alu.exec_instruction("mod b 5");
        assert_eq!(1, alu.get_val(&'b'));

        alu.exec_instruction("inp c");
        alu.exec_instruction("mod c 4");
        assert_eq!(0, alu.get_val(&'c'));

        alu.exec_instruction("inp d");
        alu.exec_instruction("mod d 5");
        assert_eq!(-3, alu.get_val(&'d'));

        alu.exec_instruction("inp e");
        alu.exec_instruction("mod e b");
        assert_eq!(0, alu.get_val(&'e'));
    }

    #[test]
    fn test_alu_eql() {
        let mut alu = Alu::init_with_inputs(vec![5]);

        alu.exec_instruction("inp a");
        alu.exec_instruction("eql a 5");
        assert_eq!(1, alu.get_val(&'a'));

        alu.exec_instruction("eql b 5");
        assert_eq!(0, alu.get_val(&'b'));

        alu.exec_instruction("eql a b");
        assert_eq!(0, alu.get_val(&'a'));
    }
}
