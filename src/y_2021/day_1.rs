pub fn entry() {
	println!("Starting day 1!");

	let depth_measurements = aoc::read_input("./resources/y_2021/day_1_input.txt", |line| {
        return line.parse::<u32>().unwrap();
    });

    println!("{:?}", depth_measurements);

    println!("Number of depth increase: {}", find_depth_increase(&depth_measurements));

    let sliding_windows = calculate_sliding_windows(&depth_measurements);

    println!("Number of depth increase with sliding windows: {}", find_depth_increase(&sliding_windows));
}

fn calculate_sliding_windows(raw_depth_measurements: &Vec<u32>) -> Vec<u32> {
    let mut sliding_windows: Vec<u32> = vec![0; raw_depth_measurements.len() - 2];

    for i in (0..).take_while(|i| i < &(&raw_depth_measurements.len() - 2)) {
        let raw_measurement_1 = raw_depth_measurements.get(i).unwrap();
        let raw_measurement_2 = raw_depth_measurements.get(i + 1).unwrap();
        let raw_measurement_3 = raw_depth_measurements.get(i + 2).unwrap();

        sliding_windows[i] = raw_measurement_1 + raw_measurement_2 + raw_measurement_3;
    }

    sliding_windows
}

fn find_depth_increase(depth_measurements: &Vec<u32>) -> u32 {
    let mut depth_increase = 0;

    let mut last_mea = depth_measurements.get(0).unwrap();
    for depth_mea in depth_measurements {
        if last_mea < depth_mea {
            depth_increase += 1;
        }

        last_mea = depth_mea;
    }

    depth_increase
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = vec!(199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263);
        
        assert_eq!(find_depth_increase(&example_input), 7);
    }

    #[test]
    fn test_very_small_input() {
        let input = vec!(199,
            200,
            208,
            206);
        
        assert_eq!(find_depth_increase(&input), 2);
    }

    #[test]
    fn test_equal_depth() {
        let input = vec!(199,
            199,
            199);
        
        assert_eq!(find_depth_increase(&input), 0);
    }

    #[test]
    fn test_only_decrease() {
        let input = vec!(199,
            198,
            187);
        
        assert_eq!(find_depth_increase(&input), 0);
    }

    #[test]
    fn test_example_input_calc_sliding_windows() {
        let example_input = vec!(199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263);

        let example_sliding_windows = vec!(607,
        618,
        618,
        617,
        647,
        716,
        769,
        792 );
        
        assert_eq!(calculate_sliding_windows(&example_input), example_sliding_windows);
    }
}