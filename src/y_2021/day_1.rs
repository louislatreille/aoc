pub fn entry() {
	println!("Starting day 1!");

	let test_ints = aoc::read_input("./resources/day_1_input.txt", |line| {
        return line.parse::<i32>().unwrap();
    });

    println!("{:?}", test_ints);
}