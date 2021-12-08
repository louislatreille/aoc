pub fn entry() {
    println!("Starting day 8!");

    let note_lines = aoc::read_input("./resources/y_2021/day_8_input.txt", move |line| {
        return parse_note_line(line);
    });

    let (uniq_segs, digits) = agg_note_lines(note_lines);

    // Part 1
    println!("Found {} simple digits", count_simple_digits(digits));
}

// TODO can this be done with AsRef??
fn parse_note_line(line: String) -> (Vec<String>, Vec<String>) {
    let mut pipe_splits = line.split(" | ");

    // nth consumes all elements up to and including the index
    let uniq_segs = match pipe_splits.nth(0) {
        Some(val) => val.split(" ").map(|s| s.to_owned()),
        None => panic!("Invalid input!"),
    };

    let digits = match pipe_splits.nth(0) {
        Some(val) => val.split(" ").map(|s| s.to_owned()),
        None => panic!("Invalid input!"),
    };

    let uniq_seg = uniq_segs.collect();
    let digits = digits.collect();

    println!("{:?} | {:?}", uniq_seg, digits);

    (uniq_seg, digits)
}

fn agg_note_lines(note_lines: Vec<(Vec<String>, Vec<String>)>) -> (Vec<String>, Vec<String>) {
    let mut uniq_segs = vec![];
    let mut digits = vec![];

    note_lines
        .iter()
        .for_each(|(current_segs, current_digits)| {
            uniq_segs.extend(current_segs.iter().map(|s| s.to_owned()));
            digits.extend(current_digits.iter().map(|s| s.to_owned()));
        });

    (uniq_segs, digits)
}

fn count_simple_digits(digits: Vec<String>) -> usize {
    let simple_digits_seg_counts = vec![2, 3, 4, 7];
    digits
        .iter()
        .filter(|digits| simple_digits_seg_counts.contains(&digits.len()))
        .count()
}
