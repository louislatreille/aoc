use std::{collections::HashSet, iter::FromIterator};

use lazy_static::lazy_static;
use regex::Regex;

pub fn entry() {
    println!("Starting day 17!");

    let target_area: (i32, i32, i32, i32) = aoc::read_input(
        "./resources/y_2021/day_17_input.txt",
        move |line| match RE.captures(&line) {
            Some(caps) => {
                let x_min = caps.name("x_min").unwrap().as_str().parse::<i32>().unwrap();
                let x_max = caps.name("x_max").unwrap().as_str().parse::<i32>().unwrap();
                let y_min = caps.name("y_min").unwrap().as_str().parse::<i32>().unwrap();
                let y_max = caps.name("y_max").unwrap().as_str().parse::<i32>().unwrap();

                (x_min, x_max, y_min, y_max)
            }
            None => panic!(),
        },
    )
    .get(0)
    .unwrap()
    .clone();

    println!("{:?}", target_area);

    let mut highest_heights = vec![];
    for i in -1000..1000 {
        for j in -1000..1000 {
            match simulate((i, j), &target_area) {
                Some(height) => highest_heights.push(((i, j), height)),
                None => continue,
            }
        }
    }

    println!("{:?}", highest_heights);

    let highest_of_highest = highest_heights
        .iter()
        .max_by(|(_, h1), (_, h2)| h1.cmp(h2))
        .unwrap();
    println!(
        "Highest height for area is {} with initial velocity {:?}",
        highest_of_highest.1, highest_of_highest.0
    );

    let initial_vel_within_area: HashSet<(i32, i32)> =
        HashSet::from_iter(highest_heights.iter().map(|(vel, _)| vel.clone()));
    println!(
        "There are {} initial velocity that reach the area.",
        initial_vel_within_area.len()
    );
}

fn simulate(initial_velocity: (i32, i32), target_area: &(i32, i32, i32, i32)) -> Option<i32> {
    let mut projectile = Projectile::init(initial_velocity.0, initial_velocity.1);
    //println!("{:?}", projectile.get_position());

    let mut hit_area = false;
    let mut highest_height = 0;

    for _ in 0..1000 {
        projectile.step();
        //println!("{:?}", projectile.get_position());

        if projectile.get_position().1 > highest_height {
            highest_height = projectile.get_position().1;
        }

        if projectile.is_in_area(target_area) {
            hit_area = true;
            println!("Projectile is in area! {:?}", projectile.get_position());
        }

        if hit_area && !projectile.is_in_area(target_area) {
            return Some(highest_height);
        }

        if projectile.get_position().0 > target_area.1
            || projectile.get_position().1 < target_area.2
        {
            return None;
        }
    }

    None
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"x=(?P<x_min>[-]*\d+)..(?P<x_max>[-]*\d+), y=(?P<y_min>[-]*\d+)..(?P<y_max>[-]*\d+)"
    )
    .unwrap();
}

struct Projectile {
    x_velocity: i32,
    y_velocity: i32,
    x_position: i32,
    y_position: i32,
}

impl Projectile {
    fn init(x_velocity: i32, y_velocity: i32) -> Projectile {
        Projectile {
            x_velocity,
            y_velocity,
            x_position: 0,
            y_position: 0,
        }
    }

    fn step(&mut self) {
        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;

        if self.x_velocity < 0 {
            self.x_velocity += 1;
        } else if self.x_velocity > 0 {
            self.x_velocity -= 1;
        }

        self.y_velocity -= 1;
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x_position, self.y_position)
    }

    fn is_in_area(&self, area: &(i32, i32, i32, i32)) -> bool {
        self.x_position >= area.0
            && self.x_position <= area.1
            && self.y_position >= area.2
            && self.y_position <= area.3
    }
}
