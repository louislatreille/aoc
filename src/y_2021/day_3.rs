use std::collections::HashMap;

pub fn entry() {
    println!("Starting day 3!");

    let inputs = aoc::read_input("./resources/y_2021/day_3_input.txt", |line| {
        return line.to_owned(); //Try with lifetime here??
    });

    let line_length = inputs.get(0).unwrap().len();
    let mut char_counts_indexes = vec![];
    for i in 0..line_length {
        char_counts_indexes.push(count_chars(&inputs, i));
    }

    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();
    for char_counts_index in &char_counts_indexes {
        gamma_rate.push(*most_common_chars(&char_counts_index).get(0).unwrap().0);
        epsilon_rate.push(*least_common_chars(&char_counts_index).get(0).unwrap().0);
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!(
        "Gamma rate: {}, Epsilon rate: {}, Power consumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let o2_generator_rating = find_o2_generator_rating(&inputs);
    let co2_scrubber_rating = find_co2_scrubber_rating(&inputs);

    let o2_generator_rating = u32::from_str_radix(&o2_generator_rating, 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!(
        "O2 generator rating: {}, CO2 scrubber rating: {}, Life support rating: {}",
        o2_generator_rating,
        co2_scrubber_rating,
        o2_generator_rating * co2_scrubber_rating
    );
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

fn most_common_chars<'a>(counts: &'a HashMap<char, u32>) -> Vec<(&'a char, &'a u32)> {
    let max = counts
        .iter()
        .max_by(|(_, n1), (_, n2)| {
            return n1.cmp(n2);
        })
        .map(|(_, n)| *n)
        .unwrap();

    counts.iter().filter(|(_, n)| **n == max).collect()
}

fn least_common_chars<'a>(counts: &'a HashMap<char, u32>) -> Vec<(&'a char, &'a u32)> {
    let min = counts
        .iter()
        .min_by(|(_, n1), (_, n2)| {
            return n1.cmp(n2);
        })
        .map(|(_, n)| *n)
        .unwrap();

    counts.iter().filter(|(_, n)| **n == min).collect()
}

fn find_o2_generator_rating(inputs: &Vec<String>) -> String {
    let mut cloned_inputs = inputs.clone();

    let mut i: usize = 0;
    while cloned_inputs.len() > 1 {
        let counts = count_chars(&cloned_inputs, i);

        let most_common_chars = most_common_chars(&counts);

        let most_common_char = match most_common_chars.len() {
            1 => most_common_chars.get(0).unwrap().0,
            2 => &'1',
            _ => panic!("Unexpected most common chars..."),
        };

        cloned_inputs = cloned_inputs
            .iter()
            .filter(|str| {
                return str.chars().nth(i).unwrap() == *most_common_char;
            })
            .map(|str| str.to_owned())
            .collect();

        if i > 1000000 {
            println!("Likely ran into an infinite loop. Aborting...");
            break;
        }

        i += 1;
    }

    cloned_inputs.get(0).unwrap().to_owned()
}

fn find_co2_scrubber_rating(inputs: &Vec<String>) -> String {
    let mut cloned_inputs = inputs.clone();

    let mut i: usize = 0;
    while cloned_inputs.len() > 1 {
        let counts = count_chars(&cloned_inputs, i);

        let least_common_chars = least_common_chars(&counts);

        let least_common_char = match least_common_chars.len() {
            1 => least_common_chars.get(0).unwrap().0,
            2 => &'0',
            _ => panic!("Unexpected most common chars..."),
        };

        cloned_inputs = cloned_inputs
            .iter()
            .filter(|str| {
                return str.chars().nth(i).unwrap() == *least_common_char;
            })
            .map(|str| str.to_owned())
            .collect();

        if i > 1000000 {
            println!("Likely ran into an infinite loop. Aborting...");
            break;
        }

        i += 1;
    }

    cloned_inputs.get(0).unwrap().to_owned()
}
