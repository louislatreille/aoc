use std::env;
mod y_2021;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!(
            "Unexpected number of arguments. Expecting 2, got {}",
            args.len()
        );
    }

    let day_number = &args[1].parse::<u32>().unwrap();
    let day_executor = match day_number {
        2021_1 => y_2021::day_1::entry,
        2021_2 => y_2021::day_2::entry,
        2021_3 => y_2021::day_3::entry,
        2021_4 => y_2021::day_4::entry,
        2021_5 => y_2021::day_5::entry,
        2021_6 => y_2021::day_6::entry,
        2021_7 => y_2021::day_7::entry,
        2021_8 => y_2021::day_8::entry,
        2021_9 => y_2021::day_9::entry,
        2021_10 => y_2021::day_10::entry,
        2021_11 => y_2021::day_11::entry,
        2021_12 => y_2021::day_12::entry,
        2021_13 => y_2021::day_13::entry,
        2021_14 => y_2021::day_14::entry,
        2021_15 => y_2021::day_15::entry,
        2021_16 => y_2021::day_16::entry,
        _ => panic!("Unknown/unimplemented challenge day"),
    };

    day_executor();
}
