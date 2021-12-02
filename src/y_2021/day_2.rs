pub fn entry() {
	println!("Starting day 1!");

	let commands = aoc::read_input("./resources/y_2021/day_2_example.txt", |line| {
        return SubmarineCommand::new(line);
    });

    let mut position = Point::new(0, 0);
    for command in &commands {
        position = position.add_command(command);
    }

    println!("End position is: {:?}, with distance: {}", position, position.calc_distance());

    let mut sub_state = SubmarineState::init();
    for command in &commands {
        sub_state.advance(command);
    }

    println!("Current submarine state: {:?}, with distance: {}", sub_state, sub_state.calc_ditance());
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point {
            x, y
        }
    }

    fn add(&self, other: &Point) -> Point {
        Point { x: other.x + self.x, y: other.y + self.y }
    }

    fn add_command(&self, command: &SubmarineCommand) -> Point {
        match command {
            SubmarineCommand::Forward(val) => Point {x: self.x + i64::from(*val), y: self.y},
            SubmarineCommand::Down(val) => Point {x: self.x, y: self.y + i64::from(*val)},
            SubmarineCommand::Up(val) => Point {x: self.x, y: self.y - i64::from(*val)},
        }
    }

    fn calc_distance(&self) -> i64 {
        (self.x * self.y).abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
enum SubmarineCommand {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl SubmarineCommand {
    fn new(str: &str) -> SubmarineCommand {
        let str_split: Vec<&str> = str.split(' ').collect();

        let unit_increase = match str_split.get(1) {
            Some(val) => {
                match val.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => panic!("Received a non-integer unit increase!")
                }
            },
            None => panic!("Invalid command!")
        };

        match str_split.get(0) {
            Some(&"forward") => SubmarineCommand::Forward(unit_increase),
            Some(&"down") => SubmarineCommand::Down(unit_increase),
            Some(&"up") => SubmarineCommand::Up(unit_increase),
            Some(_) => panic!("Unknown command!"),
            None => panic!("Invalid command!")
        }
    }
}

impl PartialEq for SubmarineCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Forward(l0), Self::Forward(r0)) => l0 == r0,
            (Self::Down(l0), Self::Down(r0)) => l0 == r0,
            (Self::Up(l0), Self::Up(r0)) => l0 == r0,
            (_, _) => false
        }
    }
}

#[derive(Debug)]
struct SubmarineState {
    current_position: Point,
    aim: i64
}

impl SubmarineState {
    fn init() -> SubmarineState {
        SubmarineState { current_position: Point::new(0, 0), aim: 0 }
    }

    fn advance(&mut self, command: &SubmarineCommand) -> &SubmarineState {
        println!("{:?}", self);

        match command {
            SubmarineCommand::Forward(val) => {
                let y = self.aim * i64::from(*val);
                self.current_position = self.current_position.add(&Point {x: i64::from(*val), y: i64::from(y)});
            }
            SubmarineCommand::Down(val) => self.aim += i64::from(*val),
            SubmarineCommand::Up(val) => self.aim -= i64::from(*val),
        };

        self
    }

    fn calc_ditance(&self) -> i64 {
        self.current_position.calc_distance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add_from_0() {
        let initial_point = Point::new(0, 0);

        assert_eq!(initial_point.add(&Point::new(5, 4)), Point::new(5, 4));
        assert_eq!(initial_point.add(&Point::new(-5, 4)), Point::new(-5, 4));
        assert_eq!(initial_point.add(&Point::new(5, -4)), Point::new(5, -4));
        assert_eq!(initial_point.add(&Point::new(0, 0)), Point::new(0, 0));
    }

    #[test]
    fn test_point_add_from_non_0() {
        let initial_point = Point::new(-5, 2);

        assert_eq!(initial_point.add(&Point::new(5, 4)), Point::new(0, 6));
        assert_eq!(initial_point.add(&Point::new(-5, 4)), Point::new(-10, 6));
        assert_eq!(initial_point.add(&Point::new(5, -4)), Point::new(0, -2));
        assert_eq!(initial_point.add(&Point::new(0, 0)), Point::new(-5, 2));
    }

    #[test]
    fn test_point_add_comand_from_0() {
        let initial_point = Point::new(0, 0);

        assert_eq!(initial_point.add_command(&SubmarineCommand::Down(5)), Point::new(0, 5));
        assert_eq!(initial_point.add_command(&SubmarineCommand::Up(5)), Point::new(0, -5));
        assert_eq!(initial_point.add_command(&SubmarineCommand::Forward(5)), Point::new(5, 0));
    }

    #[test]
    fn test_point_add_comand_from_non_0() {
        let initial_point = Point::new(4, -8);

        assert_eq!(initial_point.add_command(&SubmarineCommand::Down(5)), Point::new(4, -3));
        assert_eq!(initial_point.add_command(&SubmarineCommand::Up(5)), Point::new(4, -13));
        assert_eq!(initial_point.add_command(&SubmarineCommand::Forward(5)), Point::new(9, -8));
    }

    #[test]
    fn test_point_calc_distance() {
        assert_eq!(Point::new(5, 4).calc_distance(), 20);
        assert_eq!(Point::new(3, -9).calc_distance(), 27);
        assert_eq!(Point::new(-7, 2).calc_distance(), 14);
    }

    #[test]
    fn test_submarine_command_new() {
        assert_eq!(SubmarineCommand::new("forward 8"), SubmarineCommand::Forward(8));
        assert_eq!(SubmarineCommand::new("down 3"), SubmarineCommand::Down(3));
        assert_eq!(SubmarineCommand::new("up 2"), SubmarineCommand::Up(2));
    }

    #[test]
    fn test_submarine_state_advance() {
        let mut initial_state = SubmarineState::init();
        
        initial_state.advance(&SubmarineCommand::Down(8));
        initial_state.advance(&SubmarineCommand::Forward(2));
        initial_state.advance(&SubmarineCommand::Up(3));
        initial_state.advance(&SubmarineCommand::Forward(5));

        assert_eq!(initial_state.calc_ditance(), 287);
    }

    #[test]
    fn test_submarine_state_advance_with_aim_lower_0() {
        let mut initial_state = SubmarineState::init();
        
        initial_state.advance(&SubmarineCommand::Up(8));
        initial_state.advance(&SubmarineCommand::Forward(2));

        assert_eq!(initial_state.calc_ditance(), 32);
    }
}