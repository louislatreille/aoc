pub fn entry() {
    println!("Starting day 10!");

    let nav_subsystem_lines =
        aoc::read_input("./resources/y_2021/day_10_example.txt", move |line| {
            return line;
        });
}

//Could probably add a lifetime here
enum NavSubsystemChar {
    Opening(char),
    Closing(char),
}

impl NavSubsystemChar {
    fn new(char: &char) -> NavSubsystemChar {
        if *char == '(' || *char == '{' || *char == '[' || *char == '<' {
            return NavSubsystemChar::Opening(char.clone());
        }

        if *char == ')' || *char == '}' || *char == ']' || *char == '>' {
            return NavSubsystemChar::Closing(char.clone());
        }

        panic!("Invalid char!");
    }
}

struct NavSubsystemParser {
    chunks: Vec<NavSubsystemChar>,
    closing_chars: Vec<NavSubsystemChar>,
    valid: bool,
    complete: bool,
}

impl NavSubsystemParser {
    fn new(line: &String) -> NavSubsystemParser {
        for char in line.chars() {
            let nav_sub_char = NavSubsystemChar::new(&char);

            match nav_sub_char {
                NavSubsystemChar::Closing(c) => {}
                NavSubsystemChar::Opening(c) => {}
            }
        }
    }
}
