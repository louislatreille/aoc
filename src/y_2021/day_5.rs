use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn entry() {
    println!("Starting day 5!");

    let lines = aoc::read_input("./resources/y_2021/day_5_input.txt", move |line| {
        Line::from_str(line)
    });

    let mut crossing_points = HashSet::new();
    for line in lines.iter() {
        //println!("Line 1: {} -> {}", line.p1, line.p2);

        if !(line.is_horizontal() || line.is_vertical()) {
            continue;
        }

        for line2 in lines.iter() {
            //println!("Line 2: {} -> {}", line2.p1, line2.p2);

            if std::ptr::eq(line, line2) {
                continue;
            }

            if !(line2.is_horizontal() || line2.is_vertical()) {
                continue;
            }

            let current_cross_points = line.get_crossing_points_with(&line2);
            crossing_points.extend(current_cross_points);
        }
    }

    println!("Part 1 found {} crossing points", crossing_points.len());
    //println!("Part 1 points are: {:?}", crossing_points);

    let mut crossing_points = HashSet::new();
    for line in lines.iter() {
        //println!("Line 1: {} -> {}", line.p1, line.p2);

        if !(line.is_horizontal() || line.is_vertical() || line.is_45_diagonal()) {
            continue;
        }

        for line2 in lines.iter() {
            //println!("Line 2: {} -> {}", line2.p1, line2.p2);

            if std::ptr::eq(line, line2) {
                continue;
            }

            if !(line2.is_horizontal() || line2.is_vertical() || line.is_45_diagonal()) {
                continue;
            }

            let current_cross_points = line.get_crossing_points_with(&line2);

            crossing_points.extend(current_cross_points);
        }
    }

    println!("Part 2 found {} crossing points", crossing_points.len());

    let mut point_counts: HashMap<Point, u32> = HashMap::new();
    for line in lines.iter() {
        let points = line.to_points();

        for point in points {
            match point_counts.get_mut(&point) {
                Some(val) => *val += 1,
                None => {
                    point_counts.insert(point, 1);
                    ()
                }
            }
        }
    }

    let number_crossing = point_counts.iter().filter(|(_, count)| **count > 1).count();

    println!("Part 2 found {} crossing points", number_crossing);

    //println!("Part 2 points are: {:?}", crossing_points);
}

#[derive(Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn from_str<T: AsRef<str>>(str: T) -> Line {
        let points: Vec<Point> = str
            .as_ref()
            .split(" -> ")
            .map(|str_point| Point::from_str(str_point))
            .collect();

        Line {
            p1: points[0],
            p2: points[1],
        }
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_45_diagonal(&self) -> bool {
        !self.is_vertical() && !self.is_horizontal() && self.slope().abs() == 1.0
    }

    fn slope(&self) -> f64 {
        if self.is_vertical() {
            panic!("Vertical line, no slope!");
        }

        f64::from(self.p2.y - self.p1.y) / f64::from(self.p2.x - self.p1.x)
    }

    fn equation(&self) -> (f64, f64) {
        if self.is_horizontal() {
            return (0.0, f64::from(self.p1.y));
        }

        if self.is_vertical() {
            panic!("Vertical line, no equation!");
        }

        let slope = self.slope();
        let b = f64::from(self.p1.y) - slope * f64::from(self.p1.x);

        (slope, b)
    }

    fn get_y_from_x(&self, x: i32) -> f64 {
        if self.is_vertical() {
            panic!("Vertical line, impossible to get y from x!");
        }

        let (slope, b) = self.equation();

        slope * f64::from(x) + b
    }

    fn get_y_from_x_fract(&self, x: f64) -> f64 {
        if self.is_vertical() {
            panic!("Vertical line, impossible to get y from x!");
        }

        let (slope, b) = self.equation();

        slope * x + b
    }

    fn to_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let range;

            if self.p1.x <= self.p2.x {
                range = self.p1.x..=self.p2.x
            } else {
                range = self.p2.x..=self.p1.x
            }

            return range.map(|x| Point::new(x, self.p1.y)).collect();
        }

        if self.is_vertical() {
            let range;

            if self.p1.y <= self.p2.y {
                range = self.p1.y..=self.p2.y
            } else {
                range = self.p2.y..=self.p1.y
            }

            return range.map(|y| Point::new(self.p1.x, y)).collect();
        }

        if self.is_45_diagonal() {
            let length = (self.p2.y - self.p1.y).abs();
            let x_orientation = match i32::cmp(&self.p1.x, &self.p2.x) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            };
            let y_orientation = match i32::cmp(&self.p1.y, &self.p2.y) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            };

            return (0..=length)
                .map(|inc| {
                    Point::new(
                        self.p1.x + (inc * x_orientation),
                        self.p1.y + (inc * y_orientation),
                    )
                })
                .collect();
        }

        panic!("Unsupported line!")
    }

    fn intersect(&self, other: &Line) -> bool {
        let o1 = orientation(&self.p1, &self.p2, &other.p1);
        let o2 = orientation(&self.p1, &self.p2, &other.p2);
        let o3 = orientation(&other.p1, &other.p2, &self.p1);
        let o4 = orientation(&other.p1, &other.p2, &self.p2);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special Cases
        // p1, q1 and p2 are collinear and p2 lies on segment p1q1
        if o1 == 0 && on_segment(&self.p1, &other.p1, &self.p2) {
            return true;
        }

        // p1, q1 and q2 are collinear and q2 lies on segment p1q1
        if o2 == 0 && on_segment(&self.p1, &other.p2, &self.p2) {
            return true;
        }

        // p2, q2 and p1 are collinear and p1 lies on segment p2q2
        if o3 == 0 && on_segment(&other.p1, &self.p1, &other.p2) {
            return true;
        }

        // p2, q2 and q1 are collinear and q1 lies on segment p2q2
        if o4 == 0 && on_segment(&other.p1, &self.p2, &other.p2) {
            return true;
        }

        return false; // Doesn't fall in any of the above cases
    }

    fn intersection_point(&self, other: &Line) -> (f64, f64) {
        let self_eq = self.equation();
        let other_eq = other.equation();
        let x = (self_eq.1 - other_eq.1) / (self_eq.0 - other_eq.0);
        let y = self.get_y_from_x_fract(x);

        (x, y)
    }

    fn get_crossing_points_with(&self, other: &Line) -> Vec<Point> {
        if !self.intersect(other) {
            return vec![];
        }

        let self_points = self.to_points();

        if !self.is_45_diagonal() && !other.is_45_diagonal() {
            let other_points = other.to_points();

            return self_points
                .iter()
                .filter(|self_point| {
                    other_points
                        .iter()
                        .any(|other_point| *self_point == other_point)
                })
                .map(|point| point.to_owned())
                .collect();
        }

        if self.is_45_diagonal() && other.is_horizontal() {
            let y = self.get_y_from_x(other.p1.x);

            if y.fract() != 0.0 {
                println!(
                    "Received a fractional y when not expecting one: line {} -> {} with x {}",
                    self.p1, self.p2, other.p1.x
                );
                panic!();
            }

            return vec![Point::new(y.floor() as i32, other.p1.y)];
        } else if other.is_45_diagonal() && self.is_horizontal() {
            let y = other.get_y_from_x(self.p1.x);

            if y.fract() != 0.0 {
                println!(
                    "Received a fractional y when not expecting one: line {} -> {} with x {}",
                    other.p1, other.p2, self.p1.x
                );
                panic!();
            }

            return vec![Point::new(y.floor() as i32, self.p1.y)];
        }

        if self.is_vertical() || other.is_vertical() {
            let points_to_use;
            let line_to_use;

            if other.is_vertical() {
                points_to_use = other.to_points();
                line_to_use = self;
            } else {
                points_to_use = self.to_points();
                line_to_use = other;
            }

            let points_to_return: Vec<Point> = points_to_use
                .iter()
                .filter(|self_point| {
                    f64::from(self_point.y) == line_to_use.get_y_from_x(self_point.x)
                })
                .map(|point| point.to_owned())
                .collect();

            if points_to_return.len() > 1 {
                println!("Found more than 1 point of intersection between a diagonal and a vertical line.");
                println!("{} -> {}", self.p1, self.p2);
                println!("{} -> {}", other.p1, other.p2);
                panic!("Found more than 1 point of intersection between a diagonal and a vertical line.")
            }

            return points_to_return;
        }

        let inter = self.intersection_point(other);

        if inter.0.fract() != 0.0 || inter.1.fract() != 0.0 {
            return vec![];
        }

        return vec![Point::new(inter.0.floor() as i32, inter.1.floor() as i32)];
    }
}

fn on_segment(p1: &Point, p2: &Point, p3: &Point) -> bool {
    if p2.x <= max(p1.x, p3.x)
        && p2.x >= min(p1.x, p3.x)
        && p2.y <= max(p1.y, p3.y)
        && p2.y >= min(p1.y, p3.y)
    {
        return true;
    }

    return false;
}

fn orientation(p1: &Point, p2: &Point, p3: &Point) -> u8 {
    let val = f64::from(p2.y - p1.y) * f64::from(p3.x - p2.x)
        - f64::from(p2.x - p1.x) * f64::from(p3.y - p2.y);

    if val == 0.0 {
        return 0;
    } else if val > 0.0 {
        return 1;
    } else {
        return 2;
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
            y: 0 - str_split.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_points_horizontal() {
        let line = Line::from_str("0,9 -> 5,9");

        assert_eq!(
            vec![
                Point::new(0, -9),
                Point::new(1, -9),
                Point::new(2, -9),
                Point::new(3, -9),
                Point::new(4, -9),
                Point::new(5, -9)
            ],
            line.to_points()
        );
    }

    #[test]
    fn test_to_points_horizontal_negative() {
        let line = Line::from_str("9,4 -> 3,4");

        assert_eq!(
            vec![
                Point::new(3, -4),
                Point::new(4, -4),
                Point::new(5, -4),
                Point::new(6, -4),
                Point::new(7, -4),
                Point::new(8, -4),
                Point::new(9, -4)
            ],
            line.to_points()
        );
    }

    #[test]
    fn test_to_points_vertical() {
        let line = Line::from_str("7,0 -> 7,4");

        assert_eq!(
            vec![
                Point::new(7, -4),
                Point::new(7, -3),
                Point::new(7, -2),
                Point::new(7, -1),
                Point::new(7, -0)
            ],
            line.to_points()
        );
    }

    #[test]
    fn test_to_points_vertical_negative() {
        let line = Line::from_str("7,4 -> 7,0");

        assert_eq!(
            vec![
                Point::new(7, -4),
                Point::new(7, -3),
                Point::new(7, -2),
                Point::new(7, -1),
                Point::new(7, -0)
            ],
            line.to_points()
        );
    }

    #[test]
    fn test_to_points_45_diagonal() {
        let line = Line::from_str("2,2 -> 4,4");

        assert_eq!(
            vec![Point::new(2, -2), Point::new(3, -3), Point::new(4, -4)],
            line.to_points()
        );

        let line = Line::from_str("4,4 -> 2,2");

        assert_eq!(
            vec![Point::new(4, -4), Point::new(3, -3), Point::new(2, -2)],
            line.to_points()
        );

        let line = Line::from_str("2,4 -> 4,2");

        assert_eq!(
            vec![Point::new(2, -4), Point::new(3, -3), Point::new(4, -2)],
            line.to_points()
        );

        let line = Line::from_str("4,2 -> 2,4");

        assert_eq!(
            vec![Point::new(4, -2), Point::new(3, -3), Point::new(2, -4)],
            line.to_points()
        );
    }

    #[test]
    fn test_get_y_from_x() {
        let line = Line::from_str("2,2 -> 4,4");

        assert_eq!(-2.0, line.get_y_from_x(2));
        assert_eq!(-3.0, line.get_y_from_x(3));
        assert_eq!(-4.0, line.get_y_from_x(4));

        let line = Line::from_str("4,4 -> 2,2");

        assert_eq!(-2.0, line.get_y_from_x(2));
        assert_eq!(-3.0, line.get_y_from_x(3));
        assert_eq!(-4.0, line.get_y_from_x(4));

        let line = Line::from_str("2,4 -> 4,2");

        assert_eq!(-4.0, line.get_y_from_x(2));
        assert_eq!(-3.0, line.get_y_from_x(3));
        assert_eq!(-2.0, line.get_y_from_x(4));

        let line = Line::from_str("4,2 -> 2,4");

        assert_eq!(-4.0, line.get_y_from_x(2));
        assert_eq!(-3.0, line.get_y_from_x(3));
        assert_eq!(-2.0, line.get_y_from_x(4));
    }

    #[test]
    fn test_get_crossing_points() {
        let line1 = Line::from_str("0,9 -> 5,9");
        let line2 = Line::from_str("0,9 -> 2,9");

        assert_eq!(
            vec![Point::new(0, -9), Point::new(1, -9), Point::new(2, -9)],
            line1.get_crossing_points_with(&line2)
        );

        let line1 = Line::from_str("9,4 -> 3,4");
        let line2 = Line::from_str("3,4 -> 1,4");

        assert_eq!(
            vec![Point::new(3, -4)],
            line1.get_crossing_points_with(&line2)
        );

        let line1 = Line::from_str("1,1 -> 3,3");
        let line2 = Line::from_str("3,1 -> 3,3");

        assert_eq!(
            vec![Point::new(3, -3)],
            line1.get_crossing_points_with(&line2)
        );

        let line1 = Line::from_str("2,4 -> 4,2");
        let line2 = Line::from_str("2,2 -> 4,4");

        assert_eq!(
            vec![Point::new(3, -3)],
            line1.get_crossing_points_with(&line2)
        );

        let line1 = Line::from_str("8,0 -> 0,8");
        let line2 = Line::from_str("6,4 -> 2,0");

        assert_eq!(
            vec![Point::new(5, -3)],
            line1.get_crossing_points_with(&line2)
        );
    }
}
