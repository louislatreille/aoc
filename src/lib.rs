use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_input<F, T>(filename: &str, line_transform: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ts = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        ts.push(line_transform(&line));
    }

    ts
}
