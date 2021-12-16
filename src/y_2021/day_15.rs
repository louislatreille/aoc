use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

pub fn entry() {
    println!("Starting day 15!");

    let nodes: Vec<Vec<u32>> =
        aoc::read_input("./resources/y_2021/day_15_input.txt", move |line| {
            return line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        });

    let score = a_star((0, 0), &nodes);
    println!("{}", score);

    let extended_map = calculate_full_map(nodes);
    //println!("{:?}", extended_map);
    let start = SystemTime::now();
    let score = a_star((0, 0), &extended_map);
    let end = SystemTime::now();
    println!(
        "Score is {}, found in {} ms",
        score,
        end.duration_since(start).unwrap().as_millis()
    );
}

fn calculate_full_map(initial_map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let height = initial_map.len();

    let mut new_map = vec![];
    let mut i_increase = 0;
    for i in 0..(5 * height) {
        let mut new_row = vec![];

        for j in 0..5 {
            let initial_row = initial_map.get(i % height).unwrap();
            new_row.extend(initial_row.iter().map(|score| {
                let new_score = score + j + i_increase;

                if new_score > 9 {
                    return new_score - 9;
                } else {
                    return new_score;
                }
            }))
        }

        if (i + 1) % height == 0 {
            i_increase += 1;
        }

        new_map.push(new_row);
    }

    new_map
}

fn a_star(start: (usize, usize), nodes: &Vec<Vec<u32>>) -> u32 {
    let height = nodes.len();
    let width = nodes.get(0).unwrap().len();

    let mut discovered_nodes = HashSet::new();
    discovered_nodes.insert(start);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), u32> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<(usize, usize), u32> = HashMap::new();
    f_score.insert(start, heuristic(start, height, width));

    while !discovered_nodes.is_empty() {
        let current = discovered_nodes
            .iter()
            .min_by(|node1, node2| {
                let score1 = f_score.get(*node1).unwrap_or(&u32::MAX);
                let score2 = f_score.get(*node2).unwrap_or(&u32::MAX);

                score1.cmp(&score2)
            })
            .unwrap();

        /*println!(
            "Currently working on node {:?} which has estimated score {}",
            current.0, current.1
        );*/

        if *current == (height - 1, width - 1) {
            return *f_score.get(current).unwrap();
        }

        let current = *current;
        discovered_nodes.remove(&current);

        let neighbor_i_1 = current.0.checked_sub(1);
        let neighbor_j_1 = current.1.checked_sub(1);
        let mut neighbors = vec![
            Some((current.0 + 1, current.1)),
            Some((current.0, current.1 + 1)),
        ];

        if neighbor_i_1.is_some() {
            neighbors.push(Some((neighbor_i_1.unwrap(), current.1)));
        }

        if neighbor_j_1.is_some() {
            neighbors.push(Some((current.0, neighbor_j_1.unwrap())));
        }

        for neighbor in neighbors {
            if neighbor.is_none() {
                continue;
            }
            let neighbor = neighbor.unwrap();

            //println!("At neighbor node {:?}", neighbor);

            match nodes.get(neighbor.0) {
                Some(val) => match val.get(neighbor.1) {
                    Some(val) => {
                        let tentative_score = *g_score.entry(current).or_insert(u32::MAX) + *val;
                        let neighbor_tent_score = *g_score.entry(neighbor).or_insert(u32::MAX);

                        //println!("Current node tentative score {}", tentative_score);
                        //println!("Neighbor node tentative score {}", neighbor_tent_score);

                        if tentative_score < neighbor_tent_score {
                            //println!("Path to neighbor better than last one!");

                            came_from.insert(neighbor, current);
                            g_score.insert(neighbor, tentative_score);
                            f_score.insert(
                                neighbor,
                                tentative_score + heuristic(neighbor, height, width),
                            );

                            discovered_nodes.insert(neighbor);
                        }
                    }
                    None => continue,
                },
                None => continue,
            };
        }
    }

    panic!("Never reached goal node!");
}

fn heuristic(node: (usize, usize), height: usize, width: usize) -> u32 {
    (width - node.0 - 1 + height - node.1 - 1) as u32
}
