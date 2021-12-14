use std::collections::HashMap;

use itertools::Itertools;

pub fn entry() {
    println!("Starting day 14!");

    let lines = aoc::read_input("./resources/y_2021/day_14_input.txt", move |line| {
        return line;
    });

    let mut poly_synthetizer = PolySynthetizer::new(lines);
    println!("{:?}", poly_synthetizer);

    poly_synthetizer.apply_x_steps_efficient(40);

    println!("{:?}", poly_synthetizer.template.len());

    let element_counts = poly_synthetizer.count_elements_efficient();
    println!("{:?}", element_counts);
    let most_common = element_counts
        .iter()
        .max_by(|c1, c2| c1.1.cmp(c2.1))
        .unwrap();
    let least_common = element_counts
        .iter()
        .min_by(|c1, c2| c1.1.cmp(c2.1))
        .unwrap();

    println!(
        "Most common: {}, least common: {}. Result: {}",
        most_common.0,
        least_common.0,
        most_common.1 - least_common.1
    );
}

#[derive(Debug)]
struct PolySynthetizer {
    template: String,
    pairs: HashMap<(char, char), u64>,
    first_pair: (char, char),
    last_pair: (char, char),
    element_counts: HashMap<char, u64>,
    rules: HashMap<(char, char), char>,
}

impl PolySynthetizer {
    fn new(inputs: Vec<String>) -> PolySynthetizer {
        let template = inputs.get(0).unwrap().to_owned();

        let mut pairs = HashMap::new();
        for pair in template.chars().tuple_windows::<(char, char)>() {
            let count = pairs.entry(pair).or_insert(0_u64);
            *count += 1;
        }

        let mut rules_iter = inputs.iter();
        rules_iter.nth(1);

        let mut rules = HashMap::new();
        for rule in rules_iter {
            let mut split = rule.split(" -> ");
            let pair = split.next().unwrap();
            let element_1 = pair.chars().nth(0).unwrap();
            let element_2 = pair.chars().nth(1).unwrap();

            rules.insert(
                (element_1, element_2),
                split.next().unwrap().chars().nth(0).unwrap(),
            );
        }

        PolySynthetizer {
            first_pair: template
                .chars()
                .tuple_windows::<(char, char)>()
                .nth(0)
                .unwrap(),
            last_pair: template
                .chars()
                .tuple_windows::<(char, char)>()
                .last()
                .unwrap(),
            template,
            pairs,
            element_counts: HashMap::new(),
            rules,
        }
    }

    fn apply_step(&mut self) {
        let mut new_template = String::from("");

        for pair in self.template.chars().tuple_windows::<(char, char)>() {
            new_template.push(pair.0);

            let mut concat_pair = pair.0.to_string();
            concat_pair.push(pair.1);
            new_template.push(*self.rules.get(&pair).unwrap());

            //new_template.push(pair.1);
        }

        new_template.push(self.template.chars().last().unwrap());

        self.template = new_template;
    }

    fn apply_step_efficient(&mut self) {
        let mut new_pairs = HashMap::new();
        let mut new_counts = HashMap::new();

        for (index, (pair, pair_count)) in self.pairs.iter().enumerate() {
            //println!("Initial pair is {} x {:?}", pair.1, pair.0);

            let new_element = self.rules.get(&pair).unwrap();
            let new_pair_1 = (pair.0, *new_element);
            let new_pair_2 = (*new_element, pair.1);
            let triple = (pair.0, *new_element, pair.1);

            /*println!(
                "New element to insert is {}, creating pair {:?} and pair {:?}",
                new_element, new_pair_1, new_pair_2
            );*/

            let count = new_pairs.entry(new_pair_1).or_insert(0_u64);
            *count += pair_count;

            let count = new_pairs.entry(new_pair_2).or_insert(0_u64);
            *count += pair_count;

            //println!("New pairs are now {:?}", new_pairs);

            if *pair == self.first_pair {
                self.first_pair = new_pair_1;

                let count = new_counts.entry(triple.0).or_insert(0_u64);
                *count += pair_count;
                let count = new_counts.entry(triple.1).or_insert(0_u64);
                *count += pair_count;
                let count = new_counts.entry(triple.2).or_insert(0_u64);
                *count += pair_count;
            } else {
                let count = new_counts.entry(triple.1).or_insert(0_u64);
                *count += pair_count;
                let count = new_counts.entry(triple.2).or_insert(0_u64);
                *count += pair_count;
            }
        }

        self.pairs = new_pairs;
        self.element_counts = new_counts;
    }

    fn apply_x_steps(&mut self, amount: u32) {
        for i in 0..amount {
            println!("Applying step {}", i + 1);
            self.apply_step();
            println!(
                "Step {} done! Polymer now {} long",
                i + 1,
                self.template.len()
            );
        }
    }

    fn apply_x_steps_efficient(&mut self, amount: u32) {
        for i in 0..amount {
            println!("Applying step {}", i + 1);
            self.apply_step_efficient();
        }
    }

    fn count_elements(&self) -> HashMap<char, u64> {
        let mut counts = HashMap::new();
        for element in self.template.chars() {
            let count = counts.entry(element).or_insert(0_u64);
            *count += 1;
        }

        counts
    }

    fn count_elements_efficient(&self) -> &HashMap<char, u64> {
        &self.element_counts
    }
}
