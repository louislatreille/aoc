use std::{collections::HashMap, fmt::Display};

pub fn entry() {
    println!("Starting day 6!");

    let lines = aoc::read_input("./resources/y_2021/day_6_input.txt", move |line| {
        return line;
    });

    let fish_colony_str = match lines.get(0) {
        Some(val) => val,
        None => panic!("No fish colony line!"),
    };

    // Part 1
    let mut fish_colony = FishColony::new(fish_colony_str);
    println!("{}", fish_colony);

    while fish_colony.age < 80 {
        fish_colony.grow_1_day();
    }

    println!("{}", fish_colony);

    // Part 2
    let mut fish_colony = FishColonyOptimized::new(fish_colony_str);
    println!("{}", fish_colony);

    while fish_colony.age < 256 {
        fish_colony.grow_1_day();
        //println!("{:?}", fish_colony.fishes_by_age)
    }

    println!("{}", fish_colony);
}

struct FishColony {
    fishes: Vec<u32>,
    age: u32,
}

impl FishColony {
    fn new<T: AsRef<str>>(str: T) -> FishColony {
        let fishes = str
            .as_ref()
            .split(",")
            .map(|fish_age| fish_age.parse::<u32>().unwrap())
            .collect();

        FishColony { fishes, age: 0 }
    }

    fn grow_1_day(&mut self) {
        let mut new_fishes = 0;
        for fish in self.fishes.iter_mut() {
            if *fish == 0 {
                new_fishes += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        self.fishes.extend(vec![8; new_fishes]);
        self.age += 1;
    }

    fn size(&self) -> usize {
        self.fishes.len()
    }
}

impl Display for FishColony {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "After {} days, {} fishes: {:?}",
            self.age,
            self.size(),
            self.fishes
        )*/
        write!(f, "After {} days, {} fishes", self.age, self.size())
    }
}

struct FishColonyOptimized {
    fishes_by_age: HashMap<u32, u64>,
    age: u32,
}

impl FishColonyOptimized {
    fn new<T: AsRef<str>>(str: T) -> FishColonyOptimized {
        let mut fishes: HashMap<u32, u64> = HashMap::new();
        str.as_ref()
            .split(",")
            .map(|fish_age| fish_age.parse::<u32>().unwrap())
            .for_each(|fish| match fishes.get_mut(&fish) {
                Some(val) => *val += 1,
                None => {
                    fishes.insert(fish, 1);
                }
            });

        FishColonyOptimized {
            fishes_by_age: fishes,
            age: 0,
        }
    }

    fn grow_1_day(&mut self) {
        let mut new_colony: HashMap<u32, u64> = HashMap::new();

        for fishes in self.fishes_by_age.iter() {
            if *fishes.0 == 0 {
                match new_colony.get_mut(&8) {
                    Some(val) => *val += *fishes.1,
                    None => {
                        new_colony.insert(8, *fishes.1);
                    }
                }
                match new_colony.get_mut(&6) {
                    Some(val) => *val += *fishes.1,
                    None => {
                        new_colony.insert(6, *fishes.1);
                    }
                }
            } else {
                match new_colony.get_mut(&(fishes.0 - 1)) {
                    Some(val) => *val += *fishes.1,
                    None => {
                        new_colony.insert(fishes.0 - 1, *fishes.1);
                    }
                }
            }

            // Optimized manipulations
            /*if *fishes.0 == 0 {
                new_colony.entry(8).or_insert(*fishes.1);
                let current_6 = new_colony.entry(6).or_insert(*fishes.1);
                *current_6 += fishes.1;
            } else {
                let current = new_colony.entry(fishes.0 - 1).or_insert(*fishes.1);
                *current += fishes.1;
            }*/
        }

        self.fishes_by_age = new_colony;
        self.age += 1;
    }

    fn size(&self) -> u64 {
        let mut size = 0;

        for fishes in self.fishes_by_age.values() {
            size += *fishes;
        }

        size
    }
}

impl Display for FishColonyOptimized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "After {} days, {} fishes: {:?}",
            self.age,
            self.size(),
            self.fishes
        )*/
        write!(f, "After {} days, {} fishes", self.age, self.size())
    }
}
