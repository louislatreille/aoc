use std::collections::HashMap;

use itertools::Itertools;

pub fn entry() {
    println!("Starting day 14!");

    let lines = aoc::read_input("./resources/y_2021/day_14_example.txt", move |line| {
        return line;
    });

    let mut poly_synthetizer = PolySynthetizer::new(lines);
    println!("{:?}", poly_synthetizer);

    poly_synthetizer.apply_step();
    println!("{:?}", poly_synthetizer);
    poly_synthetizer.apply_step();
    println!("{:?}", poly_synthetizer);
    poly_synthetizer.apply_step();
    println!("{:?}", poly_synthetizer);
    poly_synthetizer.apply_step();
    println!("{:?}", poly_synthetizer);
    poly_synthetizer.apply_step();
    println!("{:?}", poly_synthetizer);

    println!("{:?}", poly_synthetizer.template.len());
}

#[derive(Debug)]
struct PolySynthetizer {
    template: String,
    rules: HashMap<String, char>,
}

impl PolySynthetizer {
    fn new(inputs: Vec<String>) -> PolySynthetizer {
        let template = inputs.get(0).unwrap().to_owned();

        let mut rules_iter = inputs.iter();
        rules_iter.nth(1);

        let mut rules = HashMap::new();
        for rule in rules_iter {
            let mut split = rule.split(" -> ");

            rules.insert(
                split.next().unwrap().to_owned(),
                split.next().unwrap().chars().nth(0).unwrap(),
            );
        }

        PolySynthetizer { template, rules }
    }

    fn apply_step(&mut self) {
        let mut new_template = String::from("");

        for pair in self.template.chars().tuple_windows::<(char, char)>() {
            new_template.push(pair.0);

            let mut concat_pair = pair.0.to_string();
            concat_pair.push(pair.1);
            new_template.push(*self.rules.get(&concat_pair).unwrap());

            //new_template.push(pair.1);
        }

        new_template.push(self.template.chars().last().unwrap());

        self.template = new_template;
    }

    fn count_elements(&self) -> HashMap<char, u32> {
        let mut counts = HashMap::new();
        for element in self.template.chars() {
            let count = counts.entry(element).or_insert(0_u32);
            *count += 1;
        }

        counts
    }
}
