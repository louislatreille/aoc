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

    let scanner_0 = scanners[0].clone();

    let mut beacons_into_scanner_0 = vec![];
    beacons_into_scanner_0.extend(scanner_0.beacons.clone());

    let mut scanner_into_0 = HashMap::new();
    scanner_into_0.insert(
        scanner_0,
        vec![(Orientation::PosxPosyPosz, Point::new(0, 0, 0))],
    );

    /*for s in scanners.iter() {
        match find_orientation_and_position(&scanner_0, &s) {
            Some((or, pos)) => {
                let ops = scanner_into_0.entry(s.clone()).or_insert(vec![]);
                ops.push((or, pos));
                //beacons_into_scanner_0.extend(transform_beacons_into(&s.beacons, &or, &pos));
            }
            None => (),
        }
    }*/

    //println!("{:?}", beacons_into_scanner_0);

    let mut scanners_2: Vec<Scanner> = scanners
        .iter()
        .filter(|s| !scanner_into_0.contains_key(*s))
        .map(|s| s.clone())
        .collect();

    let new_ops = fuck_you(&scanner_into_0, &scanners_2);
    scanner_into_0.extend(new_ops);

    //println!("{:?}", scanner_into_0);

    scanners_2 = scanners
        .iter()
        .filter(|s| !scanner_into_0.contains_key(s))
        .map(|s| s.clone())
        .collect();

    let new_ops = fuck_you(&scanner_into_0, &scanners_2);
    scanner_into_0.extend(new_ops);

    //println!("{:?}", scanner_into_0);

    let s_1 = &scanners[1];
    let s_2 = &scanners[2];
    let s_3 = &scanners[3];
    let s_4 = &scanners[4];

    let from_0_to_1 = scanner_into_0.get(&s_1).unwrap();
    let from_0_to_4 = scanner_into_0.get(&s_4).unwrap(); // Operations are not correct. Orientation is probably wrong based on the differences...

    let mut position_4 = Point::new(0, 0, 0);
    for yo in from_0_to_4 {
        println!("{:?}", yo);
        position_4 = position_4.add(&yo.1);
        println!("{}", position_4);
    }

    let test = transform_beacons_into(&s_1.beacons, scanner_into_0.get(&s_1).unwrap());
    println!("{:?}", test);

    /*while !scanners_2.is_empty() {
        let new_ops = fuck_you(&scanner_into_0, &scanners_2);
        scanner_into_0.extend(new_ops);

        println!("{:?}", scanner_into_0);

        scanners_2 = scanners
            .iter()
            .filter(|s| !scanner_into_0.contains_key(s))
            .map(|s| s.clone())
            .collect();
    }*/

    /*for entry in test.iter() {
        if entry.1.len() > 2 {
            println!("{}: {}", entry.0, entry.1.len());
        }
    }*/

    /*let test1_1 = Point::new(404, -588, -901);
    let test1_2 = Point::new(-336, 658, 858);

    let test2_1 = Point::new(528, -643, 409);
    let test2_2 = Point::new(-460, 603, -452);

    let test3_1 = Point::new(390, -675, -793);
    let test3_2 = Point::new(-322, 571, 750);

    let test4_1 = Point::new(-661, -816, -575);
    let test4_2 = Point::new(729, 430, 532);

    let test5_1 = Point::new(404, -588, -901);
    let test5_2 = Point::new(-336, 658, 858);

    println!("{}", test1_1.add(&test1_2.reorient(&Point::new(1, -1, 1))));
    println!("{}", test2_1.add(&test2_2.reorient(&Point::new(1, -1, 1))));
    println!("{}", test3_1.add(&test3_2.reorient(&Point::new(1, -1, 1))));
    println!("{}", test4_1.add(&test4_2.reorient(&Point::new(1, -1, 1))));
    println!("{}", test5_1.add(&test5_2.reorient(&Point::new(1, -1, 1))));

    let mut scanner_1_from_0 = HashSet::new();
    let mut correct_reorientation = Point::new(1, 1, 1);
    for reorientation in all_reorientations.iter() {
        scanner_1_from_0.insert(test1_1.add(&test1_2.reorient(reorientation)));
        scanner_1_from_0.insert(test2_1.add(&test2_2.reorient(reorientation)));
        scanner_1_from_0.insert(test3_1.add(&test3_2.reorient(reorientation)));
        scanner_1_from_0.insert(test4_1.add(&test4_2.reorient(reorientation)));
        scanner_1_from_0.insert(test5_1.add(&test5_2.reorient(reorientation)));

        if scanner_1_from_0.len() == 1 {
            correct_reorientation = *reorientation;
            break; // Should always happen
        }

        scanner_1_from_0.clear();
    }

    println!("{:?}, {}", scanner_1_from_0, correct_reorientation);*/
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
    let mut diffs = vec![];
    diffs.push(scanner_1.get_diffs());
    diffs.push(scanner_2.get_diffs());

    let mut simi_pairs = vec![];
    for pair in diffs.iter().combinations(2) {
        for diff_1 in pair[0] {
            for diff_2 in pair[1] {
                if diff_1.2.is_similar(&diff_2.2) {
                    simi_pairs.push((diff_1, diff_2));
                }
            }
        }
    }

    //println!("Pairs: {:?}, size: {}", simi_pairs, simi_pairs.len());

    let mut common_beacons_from_scanner_1 = HashSet::new();
    let mut common_beacons_from_scanner_2 = HashSet::new();
    for pair in simi_pairs.iter() {
        common_beacons_from_scanner_1.insert(pair.0 .0);
        common_beacons_from_scanner_1.insert(pair.0 .1);

        common_beacons_from_scanner_2.insert(pair.1 .0);
        common_beacons_from_scanner_2.insert(pair.1 .1);
    }

    /*println!(
        "Beacons from scanner 0: {:?}, size: {}",
        common_beacons_from_scanner_1,
        common_beacons_from_scanner_1.len()
    );

    println!(
        "Beacons from scanner 1: {:?}, size: {}",
        common_beacons_from_scanner_2,
        common_beacons_from_scanner_2.len()
    );*/

    let mut solver: HashMap<Point, Vec<(Point, Point)>> = HashMap::new();
    let mut found = None;
    if common_beacons_from_scanner_1.len() < 12 {
        return None;
    }

    println!(
        "Found {} common beacons between scanner {} and scanner {}!",
        common_beacons_from_scanner_1.len(),
        scanner_1.id,
        scanner_2.id
    );

    for orientation in Orientation::iter() {
        for beacon_from_scanner_0 in common_beacons_from_scanner_1.iter() {
            for beacon_from_scanner_1 in common_beacons_from_scanner_2.iter() {
                let test_point =
                    beacon_from_scanner_0.add(&beacon_from_scanner_1.reorient(&orientation));

                let vec = solver.entry(test_point).or_insert(vec![]);
                vec.push((*beacon_from_scanner_0, *beacon_from_scanner_1));
            }

            found = solver
                .iter()
                .find(|entry| entry.1.len() == common_beacons_from_scanner_1.len());

            if found.is_some() {
                println!(
                    "Found scanner position: {}, orientation is {:?} between scanner {} and scanner {}",
                    found.unwrap().0,
                    orientation,
                    scanner_1.id,
                    scanner_2.id
                );

                return Some((orientation, *found.unwrap().0));
            }
        }
    }

    None
}

fn fuck_you(
    scanner_into_0: &HashMap<Scanner, Vec<(Orientation, Point)>>,
    scanners: &Vec<Scanner>,
) -> HashMap<Scanner, Vec<(Orientation, Point)>> {
    let mut to_return: HashMap<Scanner, Vec<(Orientation, Point)>> = HashMap::new();

    for s_0 in scanner_into_0.keys() {
        for s in scanners.iter() {
            match find_orientation_and_position(s_0, &s) {
                Some((or, pos)) => {
                    let ops = to_return.entry(s.clone()).or_insert(vec![]);
                    ops.extend(scanner_into_0.get(s_0).unwrap());
                    ops.push((or, pos));

                    //beacons_into_scanner_0.extend(transform_beacons_into(&s.beacons, &or, &pos));
                }
                None => (),
            }
        }
    }

    to_return
}

fn transform_beacons_into(
    beacons: &Vec<Point>,
    operations: &Vec<(Orientation, Point)>,
) -> Vec<Point> {
    /*beacons
    .iter()
    .map(|b| {
        let mut to_return = Point::new(0, 0, 0);
        for op in operations.iter() {
            to_return = b.reorient(&op.0).add(&op.1);
        }
        to_return
    })
    .collect()*/

    println!("{:?}", operations);

    let point_1 = beacons[0];
    println!("{}", point_1);
    let mut to_return = Point::new(0, 0, 0);
    for op in operations.iter() {
        to_return = op.1.add(&point_1.reorient(&op.0));
        println!("{}", to_return);
    }

    vec![to_return]
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

#[derive(Debug, EnumIter, Clone, Copy)]
enum Orientation {
    PosxPosyPosz,
    PosxPosyNegz,
    PosxNegyPosz,
    PosxNegyNegz,
    PosxNegzPosy,
    PosxNegzNegy,
    PosxPoszPosy,
    PosxPoszNegy,

    NegxPosyPosz,
    NegxPosyNegz,
    NegxNegyPosz,
    NegxNegyNegz,
    NegxPoszPosy,
    NegxPoszNegy,
    NegxNegzPosy,
    NegxNegzNegy,

    PosyPoszPosx,
    PosyPoszNegx,
    PosyNegzPosx,
    PosyNegzNegx,
    PosyPosxNegz,
    PosyPosxPosz,
    PosyNegxPosz,
    PosyNegxNegz,

    NegyPosxPosz,
    NegyPosxNegz,
    NegyNegxPosz,
    NegyNegxNegz,
    NegyPoszPosx,
    NegyPoszNegx,
    NegyNegzPosx,
    NegyNegzNegx,

    PoszPosxPosy,
    PoszPosxNegy,
    PoszNegxPosy,
    PoszNegxNegy,
    PoszPosyPosx,
    PoszPosyNegx,
    PoszNegyPosx,
    PoszNegyNegx,

    NegzPosxPosy,
    NegzPosxNegy,
    NegzNegxPosy,
    NegzNegxNegy,
    NegzPosyPosx,
    NegzPosyNegx,
    NegzNegyPosx,
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
            Orientation::PosxNegyPosz => Point {
                x: self.x,
                y: -self.y,
                z: self.z,
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
            Orientation::PosxPosyNegz => Point {
                x: self.x,
                y: self.y,
                z: -self.z,
            },
            Orientation::PosxNegzNegy => Point {
                x: self.x,
                y: -self.z,
                z: -self.y,
            },
            Orientation::PosxPoszPosy => Point {
                x: self.x,
                y: self.z,
                z: self.y,
            },
            Orientation::NegxPosyPosz => Point {
                x: -self.x,
                y: self.y,
                z: self.z,
            },
            Orientation::NegxNegyNegz => Point {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            },
            Orientation::NegxPoszNegy => Point {
                x: -self.x,
                y: self.z,
                z: -self.y,
            },
            Orientation::NegxNegzPosy => Point {
                x: -self.x,
                y: -self.z,
                z: self.y,
            },
            Orientation::PosyPoszNegx => Point {
                x: self.y,
                y: self.z,
                z: -self.x,
            },
            Orientation::PosyNegzPosx => Point {
                x: self.y,
                y: -self.z,
                z: self.x,
            },
            Orientation::PosyPosxPosz => Point {
                x: self.y,
                y: self.x,
                z: self.z,
            },
            Orientation::PosyNegxNegz => Point {
                x: self.y,
                y: -self.x,
                z: -self.z,
            },
            Orientation::NegyPosxNegz => Point {
                x: -self.y,
                y: self.x,
                z: -self.z,
            },
            Orientation::NegyNegxPosz => Point {
                x: -self.y,
                y: -self.x,
                z: self.z,
            },
            Orientation::NegyPoszPosx => Point {
                x: -self.y,
                y: self.z,
                z: self.x,
            },
            Orientation::NegyNegzNegx => Point {
                x: -self.y,
                y: -self.z,
                z: -self.x,
            },
            Orientation::PoszPosxNegy => Point {
                x: self.z,
                y: self.x,
                z: -self.y,
            },
            Orientation::PoszNegxPosy => Point {
                x: self.z,
                y: -self.x,
                z: self.y,
            },
            Orientation::PoszPosyPosx => Point {
                x: self.z,
                y: self.y,
                z: self.x,
            },
            Orientation::PoszNegyNegx => Point {
                x: self.z,
                y: -self.y,
                z: -self.x,
            },
            Orientation::NegzPosxPosy => Point {
                x: -self.z,
                y: self.x,
                z: self.y,
            },
            Orientation::NegzNegxNegy => Point {
                x: -self.z,
                y: -self.x,
                z: -self.y,
            },
            Orientation::NegzPosyNegx => Point {
                x: -self.z,
                y: self.y,
                z: -self.x,
            },
            Orientation::NegzNegyPosx => Point {
                x: -self.z,
                y: -self.y,
                z: self.x,
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
