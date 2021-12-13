use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn entry() {
    println!("Starting day 12!");

    let split_lines: Vec<Vec<String>> =
        aoc::read_input("./resources/y_2021/day_12_example_1.txt", move |line| {
            return line.split("-").map(|str| str.to_owned()).collect();
        });

    let mut caves = HashMap::new();
    for split_line in split_lines.iter() {
        let (name, child) = match split_line
            .iter()
            .tuple_windows::<(&String, &String)>()
            .next()
        {
            Some(val) => {
                if val.1 == "start" {
                    (val.1, val.0)
                } else {
                    (val.0, val.1)
                }
            }
            None => panic!("Invalid input!"),
        };

        let cave_children = caves.entry(name).or_insert(HashSet::new());
        cave_children.insert(child);

        if name == "start" || child == "end" {
            continue;
        }

        let child_cave = caves.entry(child).or_insert(HashSet::new());
        child_cave.insert(name);
    }

    println!("{:?}", caves);

    let mut paths = vec![];
    let mut current_path = vec![String::from("start")];
    find_all_paths_bfs(&caves, &mut paths, &mut current_path, &String::from("end"));
    println!("BFS: Found {} paths", paths.len());

    /*let mut paths = vec![vec![String::from("start")]];
    find_all_paths_bfs(&caves, &mut paths, &String::from("end"));
    println!("BFS: Found {} paths", paths.len());*/
}

fn find_all_paths_bfs(
    caves: &HashMap<&String, HashSet<&String>>,
    paths: &mut Vec<Vec<String>>,
    current_path: &mut Vec<String>,
    end_node: &String,
) {
    for next_cave in find_next_caves_from_path(caves, &current_path.clone()).unwrap() {
        if *next_cave == end_node {
            let mut full_path: Vec<String> = current_path.iter().map(|s| s.to_owned()).collect();
            full_path.push(next_cave.to_owned().to_owned());
            paths.push(full_path);
            continue;
        }

        let small_caves_count = current_path
            .iter()
            .filter(|cave| cave.chars().next().unwrap().is_lowercase())
            .fold(HashMap::new(), |mut counts, cave| {
                let entry = counts.entry(cave).or_insert(0);
                *entry += 1;
                return counts;
            });
        let is_small_cave_two = small_caves_count
            .iter()
            .find(|cave_count| *cave_count.1 > 1)
            .is_some();

        if next_cave.chars().next().unwrap().is_uppercase()
            || (next_cave.chars().next().unwrap().is_lowercase() && !is_small_cave_two)
            || (next_cave.chars().next().unwrap().is_lowercase()
                && is_small_cave_two
                && !current_path.contains(next_cave))
        {
            current_path.push(next_cave.to_owned().to_owned());
            find_all_paths_bfs(caves, paths, current_path, end_node);
            current_path.pop();
        }
    }
}

/*fn find_all_paths_dfs(
    caves: &HashMap<&String, HashSet<&String>>,
    paths: &mut Vec<Vec<String>>,
    end_node: &String,
) {
    println!("{:?}", paths);
    for next_path in paths.iter_mut() {
        let clone = &next_path.clone();
        let next_caves = match find_next_caves_from_path(caves, clone) {
            Some(n) => n,
            None => continue,
        };

        for next_cave in next_caves {
            if *next_cave == end_node {
                next_path.push(next_cave.to_owned().to_owned());
                continue;
            }

            let small_caves_count = next_path
                .iter()
                .filter(|cave| cave.chars().next().unwrap().is_lowercase())
                .fold(HashMap::new(), |mut counts, cave| {
                    let entry = counts.entry(cave).or_insert(0);
                    *entry += 1;
                    return counts;
                });
            let is_small_cave_two = small_caves_count
                .iter()
                .find(|cave_count| *cave_count.1 > 1)
                .is_some();

            if next_cave.chars().next().unwrap().is_uppercase()
                || (next_cave.chars().next().unwrap().is_lowercase() && !is_small_cave_two)
                || (next_cave.chars().next().unwrap().is_lowercase()
                    && is_small_cave_two
                    && !next_path.contains(next_cave))
            {
                next_path.push(next_cave.to_owned().to_owned());
            }
        }
    }
}*/

fn find_next_caves_from_path<'a>(
    caves: &'a HashMap<&'a String, HashSet<&'a String>>,
    current_path: &'a Vec<String>,
) -> Option<&'a HashSet<&'a String>> {
    let last = match current_path.last() {
        Some(last) => last,
        None => unreachable!(),
    };

    find_next_caves(caves, last)
}

fn find_next_caves<'a>(
    caves: &'a HashMap<&'a String, HashSet<&'a String>>,
    cave: &'a String,
) -> Option<&'a HashSet<&'a String>> {
    caves.get(cave)
}
