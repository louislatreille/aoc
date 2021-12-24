use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use lazy_static::lazy_static;
use regex::Regex;

pub fn entry() {
    println!("Starting day 19!");

    let mut scanners = read_scanner_data("./resources/y_2021/day_19_input.txt");

    println!("Got {} scanners to transform", scanners.len());

    let mut scanners_with_transformed_beacons = vec![];
    let mut scanner_0 = scanners.remove(0);
    scanner_0.beacons_as_seen_from_0 = scanner_0.beacons.clone();
    scanner_0.operations_from_0 = Some((Orientation::PosxPosyPosz, Point::new(0, 0, 0)));
    scanners_with_transformed_beacons.push(scanner_0);

    let mut last_scanners_transformed = scanners_with_transformed_beacons.clone();
    while !scanners.is_empty() {
        println!("Got {} scanners to transform", scanners.len());

        let transformed_scanners =
            from_scanner_x_to_scanner_0(&last_scanners_transformed, &scanners);

        last_scanners_transformed.clear();
        last_scanners_transformed.extend(transformed_scanners.clone());
        scanners_with_transformed_beacons.extend(transformed_scanners);
        scanners.retain(|s| !scanners_with_transformed_beacons.contains(s));
    }

    println!("All scanners transformed");

    let mut all_beacons_from_0 = HashSet::new();
    for scanner in scanners_with_transformed_beacons.iter() {
        all_beacons_from_0.extend(scanner.beacons_as_seen_from_0.clone());
    }

    println!("There are {} beacons in total", all_beacons_from_0.len());

    let mut largest_distance = 0;
    for pair in scanners_with_transformed_beacons.iter().combinations(2) {
        let man_distance = pair[0]
            .operations_from_0
            .unwrap()
            .1
            .manhattan_distance_from(&pair[1].operations_from_0.unwrap().1);
        if man_distance > largest_distance {
            largest_distance = man_distance;
        }
    }

    println!("Largest distance is {}", largest_distance);
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"--- scanner (?P<id>\d+) ---").unwrap();
}

fn read_scanner_data(filename: &str) -> Vec<Scanner> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut scanners = vec![];
    let mut points = vec![];

    let mut id = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            scanners.push(Scanner::new(id, points));
            points = vec![];
            continue;
        }

        match RE.captures(&line) {
            Some(caps) => id = caps.name("id").unwrap().as_str().parse().unwrap(),
            None => (),
        }

        if line.starts_with("---") {
            continue;
        }

        points.push(Point::from_str(line));
    }

    scanners.push(Scanner::new(id, points));

    scanners
}

fn find_orientation_and_position(
    scanner_1: &Scanner,
    scanner_2: &Scanner,
) -> Option<(Orientation, Point)> {
    println!(
        "Comparing beacons from scanner {} and scanner {}",
        scanner_1.id, scanner_2.id
    );

    let beacons_1 = match !scanner_1.beacons_as_seen_from_0.is_empty() {
        true => &scanner_1.beacons_as_seen_from_0,
        false => &scanner_1.beacons,
    };

    if !scanner_2.beacons_as_seen_from_0.is_empty() {
        unreachable!();
    }

    let mut solver = HashMap::new();
    for orientation in Orientation::iter() {
        for beacon_from_scanner_0 in beacons_1.iter() {
            for beacon_from_scanner_1 in scanner_2.beacons.iter() {
                let test_point =
                    beacon_from_scanner_0.substract(&beacon_from_scanner_1.reorient(&orientation));

                let vec = solver.entry(test_point).or_insert(vec![]);
                vec.push((*beacon_from_scanner_0, *beacon_from_scanner_1));
            }

            let found = solver.iter().find(|entry| entry.1.len() >= 12);

            if found.is_some() {
                let pos_vector = found.unwrap().0;
                let count = found.unwrap().1.len();
                println!(
                    "Found {} beacons between scanner {} and scanner {}. Position: {}, orientation: {:?}",
                    count, scanner_1.id, scanner_2.id, pos_vector, orientation
                );

                return Some((orientation, *pos_vector));
            }
        }
    }

    None
}

fn from_scanner_x_to_scanner_0(
    transformed_scanners: &Vec<Scanner>,
    scanners: &Vec<Scanner>,
) -> Vec<Scanner> {
    let mut to_return = vec![];

    for s_y in transformed_scanners.iter() {
        for s in scanners.iter() {
            match find_orientation_and_position(s_y, &s) {
                Some((or, pos)) => {
                    let mut new_scanner = s.clone();
                    new_scanner.beacons_as_seen_from_0 =
                        transform_beacons_into(&new_scanner.id, &new_scanner.beacons, &(or, pos));
                    new_scanner.operations_from_0 = Some((or, pos));
                    to_return.push(new_scanner);
                }
                None => (),
            }
        }
    }

    to_return
}

fn transform_beacons_into(
    scanner_id: &u8,
    beacons: &Vec<Point>,
    operations: &(Orientation, Point),
) -> Vec<Point> {
    beacons
        .iter()
        .map(|b| operations.1.add(&b.reorient(&operations.0)))
        .collect()
}

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    beacons: Vec<Point>,
    beacons_as_seen_from_0: Vec<Point>,
    operations_from_0: Option<(Orientation, Point)>,
}

impl Scanner {
    fn new(id: u8, points: Vec<Point>) -> Scanner {
        Scanner {
            id,
            beacons: points,
            beacons_as_seen_from_0: vec![],
            operations_from_0: None,
        }
    }
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Scanner {}

impl Hash for Scanner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
enum Orientation {
    PosxPosyPosz,
    PosxNegzPosy,
    PosxNegyNegz,
    PosxPoszNegy,
    NegxNegyPosz,
    NegxPoszPosy,
    NegxPosyNegz,
    NegxNegzNegy,
    PosyPoszPosx,
    PosyNegxPosz,
    PosyNegzNegx,
    PosyPosxNegz,
    NegyNegzPosx,
    NegyPosxPosz,
    NegyPoszNegx,
    NegyNegxNegz,
    PoszPosxPosy,
    PoszNegyPosx,
    PoszNegxNegy,
    PoszPosyNegx,
    NegzNegxPosy,
    NegzPosyPosx,
    NegzPosxNegy,
    NegzNegyNegx,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn from_str<T: AsRef<str>>(str: T) -> Point {
        let mut str_split = str.as_ref().split(",");

        Point {
            x: str_split.next().unwrap().parse().unwrap(),
            y: str_split.next().unwrap().parse().unwrap(),
            z: str_split.next().unwrap().parse().unwrap(),
        }
    }

    fn substract(&self, other: &Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn reorient(&self, new_orientation: &Orientation) -> Point {
        match new_orientation {
            Orientation::PosxPosyPosz => Point {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            Orientation::PosxNegzPosy => Point {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            Orientation::PosxNegyNegz => Point {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            Orientation::PosxPoszNegy => Point {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            Orientation::NegxNegyPosz => Point {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            Orientation::NegxPoszPosy => Point {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            Orientation::NegxPosyNegz => Point {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            Orientation::NegxNegzNegy => Point {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            Orientation::PosyPoszPosx => Point {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            Orientation::PosyNegxPosz => Point {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            Orientation::PosyNegzNegx => Point {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            Orientation::PosyPosxNegz => Point {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            Orientation::NegyNegzPosx => Point {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },
            Orientation::NegyPosxPosz => Point {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            Orientation::NegyPoszNegx => Point {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            Orientation::NegyNegxNegz => Point {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            Orientation::PoszPosxPosy => Point {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            Orientation::PoszNegyPosx => Point {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            Orientation::PoszNegxNegy => Point {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            Orientation::PoszPosyNegx => Point {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            Orientation::NegzNegxPosy => Point {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            Orientation::NegzPosyPosx => Point {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            Orientation::NegzPosxNegy => Point {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            Orientation::NegzNegyNegx => Point {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
        }
    }

    fn manhattan_distance_from(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u32
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
