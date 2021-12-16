pub fn entry() {
    println!("Starting day 16!");

    let transmission = aoc::read_input("./resources/y_2021/day_16_example.txt", move |line| {
        return line.parse::<Vec<u8>>().unwrap();
    });

    println!("{:?}", transmission);
}

struct BitsPacket {
    version: [char; 3],
    type_id: [char; 3],
    sub_packets: Vec<BitsPacket>,
}
