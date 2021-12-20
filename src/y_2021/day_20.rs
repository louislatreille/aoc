use itertools::Itertools;

pub fn entry() {
    println!("Starting day 20!");

    let algorithm = aoc::read_input("./resources/y_2021/day_20_input_algo.txt", move |line| {
        return line;
    });
    let algorithm = algorithm.get(0).unwrap();

    let image = aoc::read_input("./resources/y_2021/day_20_input.txt", move |line| {
        return line;
    });

    println!("{}", algorithm);
    print_image(&image);

    let mut new_image = image;

    for i in 0..50 {
        let infinite;

        if i % 2 == 0 {
            infinite = '.';
        } else {
            infinite = get_pixel_from_algo(algorithm, 0);
        }

        new_image = enhance_image(&new_image, algorithm, infinite);

        println!("Step {} done...", i);
    }

    print_image(&new_image);
    println!("Light pixels found: {}", count_light_pixels(&new_image));
}

fn enhance_image(image: &Vec<String>, algo: &String, infinite: char) -> Vec<String> {
    let mut to_return = vec![];
    let length = image[0].len();

    for row_index in -1..=image.len() as i32 {
        let mut column_to_return = "".to_string();

        for col_index in -1..=length as i32 {
            let mut pixels = "".to_string();

            for i in row_index - 1..=row_index + 1 {
                for j in col_index - 1..=col_index + 1 {
                    //println!("{}, {}", i, j);
                    if i.is_negative() || j.is_negative() {
                        pixels.push(infinite);
                        continue;
                    }

                    let pixel = match image.get(i as usize) {
                        Some(row) => match row.chars().nth(j as usize) {
                            Some(pix) => pix,
                            None => infinite,
                        },
                        None => infinite,
                    };

                    pixels.push(pixel);
                }
            }

            //println!("{}", pixels);

            column_to_return.push(get_pixel_from_algo(algo, get_index_from_pixels(&pixels)));
        }

        to_return.push(column_to_return);
    }

    to_return
}

fn get_index_from_pixels(pixels: &String) -> usize {
    //println!("{}", pixels);

    let to_return = pixels.chars().fold("".to_string(), |sum, c| {
        let mut ret = sum;

        let to_add = match c {
            '#' => '1',
            '.' => '0',
            _ => panic!("Invalid char!"),
        };

        ret.push(to_add);
        ret
    });

    //println!("{}", to_return);

    let num = to_return
        .chars()
        .rev()
        .enumerate()
        .fold(0_usize, |sum, bit| {
            sum + (bit.1.to_digit(2).unwrap() << bit.0) as usize
        });
    //println!("{}", num);

    num
}

fn get_pixel_from_algo(algo: &String, index: usize) -> char {
    algo.chars().nth(index).unwrap()
}

fn count_light_pixels(image: &Vec<String>) -> u64 {
    let mut to_return = 0_u64;

    for row in image {
        to_return += row.matches("#").count() as u64;
    }

    to_return
}

fn print_image(image: &Vec<String>) {
    let row_length = image[0].len();
    let empty_row = (0..row_length + 6).map(|_| ".").join("");

    println!("{}", empty_row);
    println!("{}", empty_row);
    println!("{}", empty_row);
    for row in image {
        println!("...{}...", row);
    }
    println!("{}", empty_row);
    println!("{}", empty_row);
    println!("{}", empty_row);

    println!("");
}
