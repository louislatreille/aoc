use std::{collections::HashMap, slice::SliceIndex};

pub fn entry() {
    println!("Starting day 3!");

    let lines = aoc::read_input("./resources/y_2021/day_3_example.txt", |line| {
        return line.to_owned(); //Try with lifetime here??
    });

    let mut char_counts_indexes = vec![];
    for i in 0..4 {
        char_counts_indexes.push(count_chars(&lines, i));
    }

    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();
    for char_counts_index in char_counts_indexes {
        gamma_rate.push(*most_common_char(&char_counts_index).0);
        epsilon_rate.push(*least_common_char(&char_counts_index).0);
    }

    println!("Gamma rate: {}, Epsilon rate: {}", gamma_rate, epsilon_rate);
}

fn count_chars(lines: &Vec<String>, index: usize) -> HashMap<char, u32> {
    let mut counts_map: HashMap<char, u32> = HashMap::new();

    for line in lines {
        let current_char = match line.chars().nth(index) {
            Some(char) => char,
            None => '\0',
        };

        match counts_map.get_mut(&current_char) {
            Some(count) => *count += 1,
            None => {
                counts_map.insert(current_char, 1);
            }
        };
    }

    counts_map
}

fn most_common_char<'a>(counts: &'a HashMap<char, u32>) -> (&'a char, &'a u32) {
    counts
        .iter()
        .max_by(|(_, n1), (_, n2)| {
            return n1.cmp(n2);
        })
        .unwrap()
}

fn least_common_char<'a>(counts: &'a HashMap<char, u32>) -> (&'a char, &'a u32) {
    counts
        .iter()
        .min_by(|(_, n1), (_, n2)| {
            return n1.cmp(n2);
        })
        .unwrap()
}
