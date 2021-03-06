use std::{collections::HashMap, fmt::Display};

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

    let visible_points = visible_points.keys().map(|p| **p).collect();
    let grid = Grid::new(visible_points);
    grid.display(3);
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

struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        Grid { points }
    }

    fn display(&self, maximize: i32) {
        let max_x = self
            .points
            .iter()
            .max_by(|p1, p2| p1.x.cmp(&p2.x))
            .map(|p| p.x)
            .unwrap();
        let max_y = self
            .points
            .iter()
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .map(|p| p.y)
            .unwrap();

        let mut grid: Vec<Vec<String>> =
            vec![
                vec![String::from("."); ((max_x + 1) * maximize) as usize];
                ((max_y + 1) * maximize) as usize
            ];
        for point in self.points.iter() {
            let row_num = point.y * maximize;
            let col_num = point.x * maximize;
            for i in 0..maximize {
                let row = grid.get_mut((row_num + i) as usize).unwrap();

                for j in 0..maximize {
                    let val = row.get_mut((col_num + j) as usize).unwrap();
                    *val = String::from("#");
                }
            }
        }

        for row in grid {
            for point in row {
                print!("{}", point);
            }
            println!("");
        }
    }
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
