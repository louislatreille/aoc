use std::time::SystemTime;

use lazy_static::lazy_static;
use regex::Regex;

pub fn entry() {
    println!("Starting day 22!");

    let cuboids: Vec<Cuboid> =
        aoc::read_input("./resources/y_2021/day_22_input.txt", move |line| {
            return Cuboid::new(line);
        });

    let start = SystemTime::now();
    let mut cuboids_on: Vec<Cuboid> = vec![];
    for cuboid in cuboids {
        let mut new_cuboids_on = vec![];
        for cuboid_on in cuboids_on.iter() {
            new_cuboids_on.push(cuboid_on.create_intersection(&cuboid));
        }

        cuboids_on = new_cuboids_on;

        if cuboid.on {
            cuboids_on.push(cuboid);
        }
    }

    let mut total_on = 0;
    for cuboid in cuboids_on.iter() {
        total_on += cuboid.num_cubes();
    }
    println!(
        "Ran in {} ms",
        SystemTime::now().duration_since(start).unwrap().as_millis()
    );

    println!("{} cubes are on", total_on);
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?P<on>on|off) (?P<x>x=(?P<x_min>-*\d+)..(?P<x_max>-*\d+)),(?P<y>y=(?P<y_min>-*\d+)..(?P<y_max>-*\d+)),(?P<z>z=(?P<z_min>-*\d+)..(?P<z_max>-*\d+))"
    )
    .unwrap();
}

#[derive(Clone, Debug)]
struct Cuboid {
    on: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    intersections: Vec<Cuboid>,
}

impl Cuboid {
    fn new(line: String) -> Cuboid {
        let caps = match RE.captures(&line) {
            Some(caps) => caps,
            None => panic!("Invalid input!"),
        };

        let on_match = caps.name("on").unwrap();

        let mut on = false;
        if on_match.as_str().eq("on") {
            on = true;
        }

        Cuboid {
            on,
            x_min: caps.name("x_min").unwrap().as_str().parse().unwrap(),
            x_max: caps.name("x_max").unwrap().as_str().parse().unwrap(),
            y_min: caps.name("y_min").unwrap().as_str().parse().unwrap(),
            y_max: caps.name("y_max").unwrap().as_str().parse().unwrap(),
            z_min: caps.name("z_min").unwrap().as_str().parse().unwrap(),
            z_max: caps.name("z_max").unwrap().as_str().parse().unwrap(),
            intersections: vec![],
        }
    }

    fn all_1_1_1_cubes(&self) -> Vec<(i32, i32, i32)> {
        let mut to_return = vec![];

        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                for z in self.z_min..=self.z_max {
                    to_return.push((x, y, z));
                }
            }
        }

        to_return
    }

    fn create_intersection(&self, other: &Self) -> Cuboid {
        // Check if both cuboids have 0 overlap
        if self.x_max < other.x_min
            || self.y_max < other.y_min
            || self.z_max < other.z_min
            || other.x_max < self.x_min
            || other.y_max < self.y_min
            || other.z_max < self.z_min
        {
            let mut new_self = self.clone();
            return new_self;
        }

        // Always create intersections only in relation with self. Don't return anything outside
        // Everything outside should be handled by the outer layer, i.e. the outer layer should put any "on" cuboid as a whole at the end of the vector
        // The above is true for either inside or outside the create_intersection function. If the other cuboid has an intersection with self, self intersections
        // will have delt with themselves, so we add the intersection between self and other as a new intersection.

        // Self and other should always have opposite on/off
        let mut intersections = vec![];
        for intersection in self.intersections.iter() {
            intersections.push(intersection.create_intersection(other));
        }

        // If self is contained within other
        /*if self.x_min >= other.x_min
            && self.x_min <= other.x_max
            && self.x_max >= other.x_min
            && self.x_max <= other.x_max
            && self.y_min >= other.y_min
            && self.y_min <= other.y_max
            && self.y_max >= other.y_min
            && self.y_max <= other.y_max
            && self.z_min >= other.z_min
            && self.z_min <= other.z_max
            && self.z_max >= other.z_min
            && self.z_max <= other.z_max
        {}*/

        // If other is contained within self
        if other.x_min >= self.x_min
            && other.x_min <= self.x_max
            && other.x_max >= self.x_min
            && other.x_max <= self.x_max
            && other.y_min >= self.y_min
            && other.y_min <= self.y_max
            && other.y_max >= self.y_min
            && other.y_max <= self.y_max
            && other.z_min >= self.z_min
            && other.z_min <= self.z_max
            && other.z_max >= self.z_min
            && other.z_max <= self.z_max
        {
            let mut new_self = self.clone();
            new_self.intersections = intersections;
            new_self.intersections.push(Cuboid {
                on: !self.on,
                x_min: other.x_min,
                x_max: other.x_max,
                y_min: other.y_min,
                y_max: other.y_max,
                z_min: other.z_min,
                z_max: other.z_max,
                intersections: vec![],
            });
            return new_self;
        }

        /*// Contiguous in x and y
        if self.x_min == other.x_min
            && self.x_max == other.x_max
            && self.y_min == other.y_min
            && self.y_max == other.y_max
        {
            if self.on && !other.on {
                let inter_z_min = i32::max(self.z_min, other.z_min);
                let inter_z_max = i32::min(self.z_max, other.z_max);

                let intersection = Cuboid {
                    on: false,
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: inter_z_min,
                    z_max: inter_z_max,
                    intersections: vec![],
                };

                let mut new_self = self.clone();
                new_self.intersections.push(intersection);

                return vec![new_self];
            } else {
                unreachable!();
            }
        }

        // Contiguous in x and z
        if self.x_min == other.x_min
            && self.x_max == other.x_max
            && self.z_min == other.z_min
            && self.z_max == other.z_max
        {
            if self.on && !other.on {
                let inter_y_min = i32::max(self.y_min, other.y_min);
                let inter_y_max = i32::min(self.y_max, other.y_max);

                let intersection = Cuboid {
                    on: true,
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: inter_y_min,
                    y_max: inter_y_max,
                    z_min: self.z_min,
                    z_max: self.z_max,
                    intersections: vec![],
                };

                let mut new_self = self.clone();
                new_self.intersections.push(intersection);

                return vec![new_self];
            } else {
                unreachable!();
            }
        }

        // Contiguous in y and z
        if self.y_min == other.y_min
            && self.y_max == other.y_max
            && self.z_min == other.z_min
            && self.z_max == other.z_max
        {
            if self.on && !other.on {
                let inter_x_min = i32::max(self.x_min, other.x_min);
                let inter_x_max = i32::min(self.x_max, other.x_max);

                let intersection = Cuboid {
                    on: true,
                    x_min: inter_x_min,
                    x_max: inter_x_max,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: self.z_min,
                    z_max: self.z_max,
                    intersections: vec![],
                };

                let mut new_self = self.clone();
                new_self.intersections.push(intersection);

                return vec![new_self];
            } else {
                unreachable!();
            }
        }*/

        let inter_x_min = i32::max(self.x_min, other.x_min);
        let inter_x_max = i32::min(self.x_max, other.x_max);
        let inter_y_min = i32::max(self.y_min, other.y_min);
        let inter_y_max = i32::min(self.y_max, other.y_max);
        let inter_z_min = i32::max(self.z_min, other.z_min);
        let inter_z_max = i32::min(self.z_max, other.z_max);

        let intersection = Cuboid {
            on: !self.on,
            x_min: inter_x_min,
            x_max: inter_x_max,
            y_min: inter_y_min,
            y_max: inter_y_max,
            z_min: inter_z_min,
            z_max: inter_z_max,
            intersections: vec![],
        };

        let mut new_self = self.clone();
        new_self.intersections = intersections;
        new_self.intersections.push(intersection);

        return new_self;
    }

    fn num_cubes(&self) -> i128 {
        let mut self_total = (self.x_max - self.x_min + 1) as i128
            * (self.y_max - self.y_min + 1) as i128
            * (self.z_max - self.z_min + 1) as i128;

        if !self.on {
            self_total = -1 * self_total;
        }

        for inter in self.intersections.iter() {
            if self.on == inter.on {
                unreachable!();
            }

            self_total += inter.num_cubes();
        }

        self_total
    }
}
