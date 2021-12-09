pub fn entry() {
    println!("Starting day 9!");

    let height_map: Vec<Vec<u32>> =
        aoc::read_input("./resources/y_2021/day_9_input.txt", move |line| {
            return line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        });

    println!("{:?}", height_map);

    // Part 1
    let lowest_points = find_lowest_points(&height_map);
    println!("{:?}", lowest_points);

    let mut risk_level = 0;
    for lowest_point in lowest_points.iter() {
        risk_level += lowest_point.2 + 1;
    }

    println!("Risk level: {}", risk_level);

    // Part 2
    let basins = find_basins_from_lowest_points(&lowest_points, &height_map);
    println!("{:?}", basins);

    let mut largest_basin_size = (0, 0, 0);
    for basin in basins.iter() {
        if basin.len() > largest_basin_size.0 {
            largest_basin_size = (basin.len(), largest_basin_size.0, largest_basin_size.1);
            continue;
        }

        if basin.len() > largest_basin_size.1 {
            largest_basin_size = (largest_basin_size.0, basin.len(), largest_basin_size.1);
            continue;
        }

        if basin.len() > largest_basin_size.2 {
            largest_basin_size = (largest_basin_size.0, largest_basin_size.1, basin.len());
            continue;
        }
    }

    println!("{:?}", largest_basin_size);

    println!(
        "Answer: {}",
        largest_basin_size.0 * largest_basin_size.1 * largest_basin_size.2
    );
}

fn find_lowest_points(height_map: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut to_return = vec![];

    let vert_size = height_map.len();
    let hor_size = height_map[0].len();

    for i in 0..vert_size {
        for j in 0..hor_size {
            let current_height = height_map[i][j];
            let lower_than_top = match height_map.get(i - 1) {
                Some(val) => match val.get(j) {
                    Some(val) => *val > current_height,
                    None => true,
                },
                None => true,
            };

            let lower_than_left = match height_map.get(i) {
                Some(val) => match val.get(j - 1) {
                    Some(val) => *val > current_height,
                    None => true,
                },
                None => true,
            };

            let lower_than_bottom = match height_map.get(i + 1) {
                Some(val) => match val.get(j) {
                    Some(val) => *val > current_height,
                    None => true,
                },
                None => true,
            };

            let lower_than_right = match height_map.get(i) {
                Some(val) => match val.get(j + 1) {
                    Some(val) => *val > current_height,
                    None => true,
                },
                None => true,
            };

            if lower_than_top && lower_than_left && lower_than_bottom && lower_than_right {
                to_return.push((i, j, current_height));
            }
        }
    }

    to_return
}

fn find_basins_from_lowest_points<'a>(
    lowest_points: &'a Vec<(usize, usize, u32)>,
    height_map: &'a Vec<Vec<u32>>,
) -> Vec<Vec<(usize, usize, u32)>> {
    let mut basins = vec![];

    for lowest_point in lowest_points {
        let mut basin = vec![];
        find_basin_from_point(lowest_point, &mut basin, height_map);
        basins.push(basin);
    }

    basins
}

fn find_basin_from_point<'a>(
    point: &'a (usize, usize, u32),
    basin: &'a mut Vec<(usize, usize, u32)>,
    height_map: &'a Vec<Vec<u32>>,
) -> &'a Vec<(usize, usize, u32)> {
    //println!("{:?}", point);
    //println!("{:?}", basin);

    if point.2 == 9 {
        return basin;
    } else if basin.contains(point) {
        return basin;
    }

    basin.push(*point);

    let i = point.0;
    let j = point.1;

    match height_map.get(i - 1) {
        Some(val) => match val.get(j) {
            Some(val) => {
                if basin.contains(&(i - 1, j, *val)) {
                    ()
                }

                find_basin_from_point(&(i - 1, j, *val), basin, height_map);
            }
            None => (),
        },
        None => (),
    };

    match height_map.get(i) {
        Some(val) => match val.get(j - 1) {
            Some(val) => {
                if basin.contains(&(i, j - 1, *val)) {
                    ()
                }

                find_basin_from_point(&(i, j - 1, *val), basin, height_map);
            }
            None => (),
        },
        None => (),
    };

    match height_map.get(i + 1) {
        Some(val) => match val.get(j) {
            Some(val) => {
                if basin.contains(&(i + 1, j, *val)) {
                    ()
                }

                find_basin_from_point(&(i + 1, j, *val), basin, height_map);
            }
            None => (),
        },
        None => (),
    };

    match height_map.get(i) {
        Some(val) => match val.get(j + 1) {
            Some(val) => {
                if basin.contains(&(i, j + 1, *val)) {
                    ()
                }

                find_basin_from_point(&(i, j + 1, *val), basin, height_map);
            }
            None => (),
        },
        None => (),
    };

    /*to_return.extend(basin_points_top.iter().map(|point| point.to_owned()));
    to_return.extend(basin_points_left.iter().map(|point| point.to_owned()));
    to_return.extend(basin_points_bottom.iter().map(|point| point.to_owned()));
    to_return.extend(basin_points_right.iter().map(|point| point.to_owned()));*/

    basin
}
