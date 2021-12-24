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

    let scanners = read_scanner_data("./resources/y_2021/day_19_example.txt");

    println!("Got {} scanners to transform", scanners.len());

    let scanner_0 = scanners[0].clone();

    let mut beacons_into_scanner_0 = vec![];
    beacons_into_scanner_0.extend(scanner_0.beacons.clone());

    let mut scanner_into_0 = HashMap::new();
    scanner_into_0.insert(
        scanner_0,
        vec![(Orientation::PosxPosyPosz, Point::new(0, 0, 0))],
    );

    let mut scanners_not_transformed = scanners.clone();
    let mut last_scanners_found = HashMap::new();
    last_scanners_found.extend(scanner_into_0.clone());

    while scanners_not_transformed.len() > 0 {
        println!(
            "Got {} scanners to transform",
            scanners_not_transformed.len()
        );

        scanners_not_transformed = scanners
            .iter()
            .filter(|s| !scanner_into_0.contains_key(*s))
            .map(|s| s.clone())
            .collect();

        let new_ops = from_scanner_x_to_scanner_0(&last_scanners_found, &scanners_not_transformed);
        last_scanners_found.clear();
        last_scanners_found.extend(new_ops.clone());
        scanner_into_0.extend(new_ops.clone());
    }

    println!("All scanners transformed");

    let mut all_beacons_from_scanner_0 = HashSet::new();
    for ops in scanner_into_0.iter() {
        println!(
            "Transforming beacons from scanner {} to scanner 0",
            ops.0.id
        );

        let new_beacons = transform_beacons_into(&ops.0.id, &ops.0.beacons, ops.1);
        all_beacons_from_scanner_0.extend(new_beacons);

        //println!("{:?}", all_beacons_from_scanner_0);
    }

    println!("{}", scanner_into_0.len());
    println!("{}", all_beacons_from_scanner_0.len());
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

    let mut solver = HashMap::new();
    let mut found;
    for orientation in Orientation::iter() {
        for beacon_from_scanner_0 in scanner_1.beacons.iter() {
            for beacon_from_scanner_1 in scanner_2.beacons.iter() {
                let test_point =
                    beacon_from_scanner_0.substract(&beacon_from_scanner_1.reorient(&orientation));

                let vec = solver.entry(test_point).or_insert(vec![]);
                vec.push((*beacon_from_scanner_0, *beacon_from_scanner_1, orientation));
            }
        }
    }

    found = solver.iter().filter(|entry| entry.1.len() >= 12).count();

    println!("Found {} set of 12 or more points", found);

    if found > 0 {
        let answer = solver
            .iter()
            .filter(|entry| entry.1.len() >= 12)
            .last()
            .unwrap();
        println!(
            "Found scanner position: {}, orientation is {:?} between scanner {} and scanner {}",
            answer.0, answer.1[0].2, scanner_1.id, scanner_2.id
        );

        return Some((answer.1[0].2, *answer.0));
    }

    None
}

fn from_scanner_x_to_scanner_0(
    scanner_into_0: &HashMap<Scanner, Vec<(Orientation, Point)>>,
    scanners: &Vec<Scanner>,
) -> HashMap<Scanner, Vec<(Orientation, Point)>> {
    let mut to_return: HashMap<Scanner, Vec<(Orientation, Point)>> = HashMap::new();

    for s_y in scanner_into_0.keys() {
        for s in scanners.iter() {
            match find_orientation_and_position(s_y, &s) {
                Some((or, pos)) => {
                    let ops_between_scanner_y_and_0 = scanner_into_0.get(s_y).unwrap()[0];
                    println!(
                        "Operations between scanner 0 and scanner {}: vector {}, rotation {:?}",
                        s_y.id, ops_between_scanner_y_and_0.1, ops_between_scanner_y_and_0.0
                    );
                    println!("Applying rotation {:?} to vector found between scanner {} and scanner {} and adding to {}", ops_between_scanner_y_and_0.0, s_y.id, s.id, ops_between_scanner_y_and_0.1);
                    let ops = to_return.entry(s.clone()).or_insert(vec![]);
                    ops.push((
                        or,
                        ops_between_scanner_y_and_0
                            .1
                            .add(&pos.reorient(&ops_between_scanner_y_and_0.0)),
                    ));
                    println!(
                        "Operations between scanner 0 and scanner {}: vector {}, rotation {:?}",
                        s.id, ops[0].1, ops[0].0
                    );
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
    operations: &Vec<(Orientation, Point)>,
) -> Vec<Point> {
    println!("Operations {:?}", operations);

    let ops = operations[0];

    let new_points = beacons
        .iter()
        .map(|b| ops.1.add(&b.reorient(&ops.0)))
        .collect();

    println!("{:?}", new_points);

    new_points
}

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    beacons: Vec<Point>,
}

impl Scanner {
    fn new(id: u8, points: Vec<Point>) -> Scanner {
        Scanner {
            id,
            beacons: points,
        }
    }

    fn get_diffs(&self) -> Vec<(Point, Point, Point)> {
        let mut diffs = vec![];

        for pair in self.beacons.iter().combinations(2) {
            diffs.push((pair[0].clone(), pair[1].clone(), pair[0].substract(pair[1])));
        }

        diffs
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

    fn diff_references(&self) -> Vec<Point> {
        let mut to_return = vec![];

        to_return.push(self.clone());
        to_return.push(Point::new(self.x, self.z, self.y));
        to_return.push(Point::new(self.y, self.x, self.z));
        to_return.push(Point::new(self.y, self.z, self.x));
        to_return.push(Point::new(self.z, self.x, self.y));
        to_return.push(Point::new(self.z, self.y, self.x));

        to_return
    }

    fn is_similar(&self, other: &Self) -> bool {
        (self.x.abs() == other.x.abs()
            || self.x.abs() == other.y.abs()
            || self.x.abs() == other.z.abs())
            && (self.y.abs() == other.x.abs()
                || self.y.abs() == other.y.abs()
                || self.y.abs() == other.z.abs())
            && (self.z.abs() == other.x.abs()
                || self.z.abs() == other.y.abs()
                || self.z.abs() == other.z.abs())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
