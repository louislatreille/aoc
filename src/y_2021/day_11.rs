use std::fmt::Display;

pub fn entry() {
    println!("Starting day 11!");

    let lines = aoc::read_input("./resources/y_2021/day_11_input.txt", move |line| {
        return line;
    });

    let mut octo_grid = OctoGrid::new(&lines);
    println!("{}", octo_grid);

    let mut last_num_flases = 0;
    for step in 1..=1000 {
        octo_grid.step();
        //println!("{}", octo_grid);
        if octo_grid.flashes - last_num_flases == 100 {
            println!("Big flash! Step: {}", step);
        }
        last_num_flases = octo_grid.flashes;
        octo_grid.unmute_grid();
    }

    println!("Total flashes: {}", octo_grid.flashes);
}

struct OctoGrid {
    dumbo_octos: Vec<Vec<DumboOcto>>,
    size: usize,
    flashes: u32,
}

impl OctoGrid {
    fn new(lines: &Vec<String>) -> OctoGrid {
        let mut grid = vec![];

        for line in lines {
            let mut row = vec![];
            for num in line.chars() {
                row.push(DumboOcto::new(num.to_digit(10).unwrap()));
            }
            grid.push(row);
        }

        OctoGrid {
            dumbo_octos: grid,
            size: lines.len(),
            flashes: 0,
        }
    }

    fn step(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.dumbo_octos.get_mut(i) {
                    Some(row) => match row.get_mut(j) {
                        Some(octo) => {
                            if octo.energize() {
                                self.flashes += 1;
                                self.local_energy_increase(i, j);
                            }
                        }
                        None => (),
                    },
                    None => (),
                }
            }
        }
    }

    fn local_energy_increase(&mut self, i: usize, j: usize) {
        let x_low = match i.checked_sub(1) {
            Some(val) => val,
            None => i,
        };
        let y_low = match j.checked_sub(1) {
            Some(val) => val,
            None => j,
        };

        for ii in x_low..=i + 1 {
            for jj in y_low..=j + 1 {
                match self.dumbo_octos.get_mut(ii) {
                    Some(row) => match row.get_mut(jj) {
                        Some(octo) => {
                            if octo.energize() {
                                self.flashes += 1;
                                self.local_energy_increase(ii, jj);
                            }
                        }
                        None => (),
                    },
                    None => (),
                }
            }
        }
    }

    fn unmute_grid(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.dumbo_octos.get_mut(i) {
                    Some(row) => match row.get_mut(j) {
                        Some(octo) => {
                            octo.unmute();
                        }
                        None => (),
                    },
                    None => (),
                }
            }
        }
    }
}

impl Display for OctoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.dumbo_octos.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

struct DumboOcto {
    energy_level: u32,
    muted: bool,
}

impl DumboOcto {
    fn new(energy_level: u32) -> DumboOcto {
        DumboOcto {
            energy_level,
            muted: false,
        }
    }

    fn energize(&mut self) -> bool {
        if self.muted {
            return false;
        }

        self.energy_level += 1;

        if self.energy_level > 9 {
            self.muted = true;
            self.energy_level = 0;
            return true;
        }

        return false;
    }

    fn unmute(&mut self) {
        self.muted = false;
    }
}

impl Display for DumboOcto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.energy_level)
    }
}
