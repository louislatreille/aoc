use std::collections::HashMap;

use itertools::Itertools;

pub fn entry() {
    println!("Starting day 7!");

    let lines = aoc::read_input("./resources/y_2021/day_7_input.txt", move |line| {
        return line;
    });

    let values = &lines[0];
    let values: Vec<u32> = values
        .split(",")
        .map(|value| value.parse().unwrap())
        .collect();

    let (point, fuel) = calc_center_point(&values);

    println!("Center point is {}, with fuel: {}", point, fuel);

    let median = calc_median(&values);
    println!(
        "Median is {}, with fuel: {}",
        median,
        calc_weight_from_point(&values, median)
    );

    // Part 2
    let (point, fuel) = calc_weight_from_point_exp(&values);
    println!("Center point is {}, with fuel: {}", point, fuel);
}

fn calc_median(numbers: &Vec<u32>) -> u32 {
    let mut numbers = numbers.clone();
    numbers.sort();

    let median = match numbers.len() % 2 == 0 {
        true => {
            let mid1 = (numbers.len() - 1) / 2;
            let mid2 = mid1 + 1;

            let median = (numbers[mid1] + numbers[mid2]) / 2;

            median
        }
        false => numbers[numbers.len() / 2],
    };

    median
}

fn calc_center_point(numbers: &Vec<u32>) -> (u32, i64) {
    let min_max = numbers.iter().minmax();
    let (min, max) = match min_max {
        itertools::MinMaxResult::NoElements => panic!("Empty list of values!"),
        itertools::MinMaxResult::OneElement(_) => panic!("Only one value!"),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    let mut weight_by_points: HashMap<u32, i64> = HashMap::new();

    for i in *min..=*max {
        weight_by_points.insert(i, calc_weight_from_point(numbers, i));
    }

    println!("{:?}", weight_by_points);

    match weight_by_points
        .iter()
        .min_by(|(_, weight1), (_, weight2)| weight1.cmp(weight2))
    {
        Some((point, weight)) => (*point, *weight),
        None => panic!("No data!"),
    }
}

fn calc_weight_from_point(numbers: &Vec<u32>, point: u32) -> i64 {
    let mut weight = 0;

    for number in numbers.iter() {
        weight += (i64::from(*number) - i64::from(point)).abs();
    }

    weight
}

fn calc_weight_from_point_exp(numbers: &Vec<u32>) -> (u32, u32) {
    let min_max = numbers.iter().minmax();
    let (min, max) = match min_max {
        itertools::MinMaxResult::NoElements => panic!("Empty list of values!"),
        itertools::MinMaxResult::OneElement(_) => panic!("Only one value!"),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    let mut total_cost_by_steps: HashMap<i64, u32> = HashMap::new();
    for i in *min..=(*max - *min) {
        if i == 0 {
            total_cost_by_steps.insert(0, 0);
            continue;
        }

        let prev_cost = match total_cost_by_steps.get(&i64::from(i - 1)) {
            Some(val) => val,
            None => unreachable!(),
        };

        total_cost_by_steps.insert(i64::from(i), prev_cost + i);
    }

    println!("{:?}", total_cost_by_steps);

    let mut weight_by_points: HashMap<u32, u32> = HashMap::new();

    for i in *min..=*max {
        let mut weight = 0;

        for number in numbers.iter() {
            weight += match total_cost_by_steps.get(&(i64::from(*number) - i64::from(i)).abs()) {
                Some(val) => val,
                None => unreachable!(),
            }
        }

        weight_by_points.insert(i, weight);
    }

    println!("{:?}", weight_by_points);

    match weight_by_points
        .iter()
        .min_by(|(_, weight1), (_, weight2)| weight1.cmp(weight2))
    {
        Some((point, weight)) => (*point, *weight),
        None => panic!("No data!"),
    }
}
