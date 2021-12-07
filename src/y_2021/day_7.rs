use itertools::Itertools;

pub fn entry() {
    println!("Starting day 7!");

    let lines = aoc::read_input("./resources/y_2021/day_7_input.txt", move |line| {
        return line;
    });

    let values = &lines[0];
    let mut values: Vec<u32> = values
        .split(",")
        .map(|value| value.parse().unwrap())
        .collect();

    let min_max = values.iter().minmax();
    let (min, max) = match min_max {
        itertools::MinMaxResult::NoElements => panic!("Empty list of values!"),
        itertools::MinMaxResult::OneElement(_) => panic!("Only one value!"),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    println!("Min: {}, max: {}", min, max);
    println!("Total amount of values: {}", values.len());

    values.sort();
    let mid = values.len() / 2;
}
