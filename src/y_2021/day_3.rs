use std::collections::HashMap;

pub fn entry() {
    println!("Starting day 3!");

    let inputs = aoc::read_input("./resources/y_2021/day_3_input.txt", move |line| {
        return line;
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

fn find_o2_generator_rating<'b>(inputs: &Vec<String>) -> String {
    find_rating_with_discriminator(inputs, |counts| {
        let chars = most_common_chars(counts);

        match chars.len() {
            1 => chars.get(0).unwrap().0,
            2 => &'1',
            _ => panic!("Unexpected most common chars..."),
        }
    })
}

fn find_co2_scrubber_rating<'b>(inputs: &Vec<String>) -> String {
    find_rating_with_discriminator(inputs, |counts| {
        let chars = least_common_chars(counts);

        match chars.len() {
            1 => chars.get(0).unwrap().0,
            2 => &'0',
            _ => panic!("Unexpected most common chars..."),
        }
    })
}

fn find_rating_with_discriminator<'b, F>(inputs: &Vec<String>, discriminator: F) -> String
where
    F: for<'a> Fn(&'a HashMap<char, u32>) -> &'a char,
{
    let mut cloned_inputs = inputs.clone();

    let mut i: usize = 0;
    while cloned_inputs.len() > 1 {
        let counts = count_chars(&cloned_inputs, i);

        let least_common_char = discriminator(&counts);

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

    cloned_inputs.get(0).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_o2_generator_rating_example() {
        let inputs = vec![
            "00100".to_owned(),
            "11110".to_owned(),
            "10110".to_owned(),
            "10111".to_owned(),
            "10101".to_owned(),
            "01111".to_owned(),
            "00111".to_owned(),
            "11100".to_owned(),
            "10000".to_owned(),
            "11001".to_owned(),
            "00010".to_owned(),
            "01010".to_owned(),
        ];

        assert_eq!("10111", find_o2_generator_rating(&inputs));
    }

    #[test]
    fn test_find_co2_scrubber_rating_example() {
        let inputs = vec![
            "00100".to_owned(),
            "11110".to_owned(),
            "10110".to_owned(),
            "10111".to_owned(),
            "10101".to_owned(),
            "01111".to_owned(),
            "00111".to_owned(),
            "11100".to_owned(),
            "10000".to_owned(),
            "11001".to_owned(),
            "00010".to_owned(),
            "01010".to_owned(),
        ];

        assert_eq!("01010", find_co2_scrubber_rating(&inputs));
    }
}
