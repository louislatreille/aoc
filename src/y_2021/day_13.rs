use std::{collections::HashMap, fmt::Display, io::Lines};

pub fn entry() {
    println!("Starting day 13!");

    let lines = aoc::read_input("./resources/y_2021/day_13_input.txt", move |line| {
        return line;
    });

    let (points, folds) = read_input(lines);
    //println!("Points: {:?}", points);
    //println!("Folds: {:?}", folds);

    let first_fold = folds.iter().nth(0).unwrap();
    let mut final_points = vec![];
    for point in points.iter() {
        final_points.push(point.apply_fold(first_fold));
    }

    //println!("Folded points: {:?}", final_points);

    let mut visible_points = HashMap::new();
    for point in final_points.iter() {
        let num = visible_points.entry(point).or_insert(0);
        *num += 1;
    }

    println!("Visible points: {:?}", visible_points.len());

    let mut final_points = vec![];
    let mut points_clone = points.clone();
    for fold in folds.iter() {
        final_points.clear();
        for point in points_clone.iter() {
            final_points.push(point.apply_fold(fold));
        }
        points_clone = final_points.clone();
    }

    //println!("Folded points: {:?}", final_points);

    let mut visible_points = HashMap::new();
    for point in final_points.iter() {
        let num = visible_points.entry(point).or_insert(0);
        *num += 1;
    }

    println!("Visible points: {:?}", visible_points.len());

    let visible_points = visible_points.keys();
    let mut grid: Vec<Vec<String>> = vec![vec![String::from(" "); 50]; 50];
    for visible_point in visible_points {
        let row = grid.get_mut(visible_point.y as usize).unwrap();
        let val = row.get_mut(visible_point.x as usize).unwrap();
        *val = String::from("#");
    }

    for row in grid.iter() {
        println!("{:?}", row);
    }
}

fn read_input(lines: Vec<String>) -> (Vec<Point>, Vec<Fold>) {
    let mut is_folds = false;
    let mut folds = vec![];
    let mut points = vec![];

    for line in lines {
        if line.is_empty() {
            is_folds = true;
            continue;
        }

        if is_folds {
            let mut splits = line.split(" ");
            folds.push(Fold::new(splits.nth(2).unwrap()));
        } else {
            points.push(Point::from_str(line));
        }
    }

    (points, folds)
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    fn new(str: &str) -> Fold {
        let mut split = str.split("=");

        match split.next() {
            Some("x") => {
                let num = match split.next() {
                    Some(num) => num.parse::<i32>().unwrap(),
                    None => panic!("Invalid input!"),
                };

                Fold::X(num)
            }
            Some("y") => {
                let num = match split.next() {
                    Some(num) => num.parse::<i32>().unwrap(),
                    None => panic!("Invalid input!"),
                };

                Fold::Y(num)
            }
            _ => panic!("Invalid input!"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn from_str<T: AsRef<str>>(str: T) -> Point {
        let mut str_split = str.as_ref().split(",");

        Point {
            x: str_split.next().unwrap().parse().unwrap(),
            y: str_split.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    fn apply_fold(&self, fold: &Fold) -> Point {
        match fold {
            Fold::X(val) => {
                if self.x > *val {
                    let fold_val = (self.x - val) * 2;
                    return self.apply_translation_x(-fold_val);
                }

                return *self;
            }
            Fold::Y(val) => {
                if self.y > *val {
                    let fold_val = (self.y - val) * 2;
                    return self.apply_translation_y(-fold_val);
                }

                return *self;
            }
        }
    }

    fn apply_translation_x(&self, val: i32) -> Point {
        Point {
            x: self.x + val,
            y: self.y,
        }
    }

    fn apply_translation_y(&self, val: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + val,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
