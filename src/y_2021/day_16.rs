pub fn entry() {
    println!("Starting day 16!");

    let transmission: Vec<String> =
        aoc::read_input("./resources/y_2021/day_16_input.txt", move |line| {
            return line;
        });

    let transmission = transmission.get(0).unwrap();

    let packet = transmission.clone();
    let bytes: Vec<u32> = transmission
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .collect();

    let mut bits = "".to_string();
    for character in bytes.iter() {
        bits += &format!("{:04b}", character);
    }

    println!("{}", transmission);
    println!("{:?}", bytes);

    let packet = BitsPacket::new(&bits);
    let versions = packet.0.get_versions();
    println!("{:?}", versions);
    let sum: u32 = versions.iter().sum();
    println!("Version sum: {}", sum);

    let value = packet.0.get_value();
    println!("Value is: {}", value);
}

struct BitsPacket {
    packet: String,
    version: String,
    version_num: u32,
    type_id: String,
    type_id_num: u32,
    sub_packets: Vec<BitsPacket>,
    payload: String,
}

impl BitsPacket {
    fn new(bits: &String) -> (BitsPacket, usize) {
        println!("Starting decoding of new packet...");
        println!("{}", bits);

        let version = bits[0..3].to_owned();
        let type_id = bits[3..6].to_owned();
        //println!("{}", version);
        //println!("{}", type_id);

        let version_num = version
            .chars()
            .rev()
            .enumerate()
            .fold(0, |sum, bit| sum + (bit.1.to_digit(2).unwrap() << bit.0));
        let type_id_num = type_id
            .chars()
            .rev()
            .enumerate()
            .fold(0, |sum, bit| sum + (bit.1.to_digit(2).unwrap() << bit.0));

        println!(
            "Found packet of type {} with version {}!",
            type_id_num, version_num
        );

        let mut packet = "".to_string();
        packet += &version;
        packet += &type_id;

        let mut packet_end = 0;
        packet_end += 3 + 3;

        let mut sub_packets = vec![];
        let mut payload = "".to_string();
        if type_id_num == 4 {
            let mut first_bit = '1';
            let mut i = 0;
            while first_bit == '1' {
                let literal = bits[6 + 5 * i..6 + 5 * (i + 1)].to_owned();
                first_bit = literal.chars().nth(0).unwrap();
                payload += &literal;
                i += 1;
            }

            packet_end += payload.len();

            packet += &payload;
            /*println!("{}", len);

            while len % 4 != 0 {
                let padding = bits[len..len + 1].to_owned();
                payload += &padding;
                len += 1;
            }*/
        } else {
            let length_type_id = bits[6..7].to_owned();
            let length_type_id_num = length_type_id.chars().nth(0).unwrap().to_digit(2).unwrap();

            packet += &length_type_id;
            packet_end += 1;

            println!("Packet length type is {}", length_type_id_num);

            if length_type_id_num == 0 {
                let length = bits[7..22].to_owned();
                let length_num = length
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(0, |sum, bit| sum + (bit.1.to_digit(2).unwrap() << bit.0));

                println!("Packet length is {}", length_num);

                packet += &length;

                let mut rest = bits[22..].to_owned();
                let mut read: usize = 0;

                while read != length_num as usize {
                    let bits_packet = BitsPacket::new(&rest);
                    rest = rest[bits_packet.1..].to_owned();
                    payload += &bits_packet.0.packet;
                    read += bits_packet.1;
                    sub_packets.push(bits_packet.0);
                }

                packet_end += 15 + length_num as usize;
                packet += &payload;
            } else {
                let length = bits[7..18].to_owned();
                let length_num = length
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(0, |sum, bit| sum + (bit.1.to_digit(2).unwrap() << bit.0));

                println!("Packet contains {} inner packets", length_num);

                packet += &length;

                let mut rest = bits[18..].to_owned();

                for _ in 0..length_num {
                    let bits_packet = BitsPacket::new(&rest);
                    rest = rest[bits_packet.1..].to_owned();
                    payload += &bits_packet.0.packet;
                    sub_packets.push(bits_packet.0);
                }

                packet_end += 11 + payload.len();
                packet += &payload;
            }
        }

        println!("Full packet is {}", packet);
        println!("Full packet length is {}", packet_end);
        println!("Packet payload is {}", payload);

        (
            BitsPacket {
                packet,
                version,
                version_num,
                type_id,
                type_id_num,
                sub_packets,
                payload,
            },
            packet_end,
        )
    }

    fn get_versions(&self) -> Vec<u32> {
        if self.type_id_num == 4 {
            return vec![self.version_num];
        }

        let mut versions = vec![self.version_num];
        for sub_packet in self.sub_packets.iter() {
            versions.extend(sub_packet.get_versions());
        }

        versions
    }

    fn get_value(&self) -> u64 {
        if self.type_id_num == 4 {
            let mut value_str = "".to_string();
            let mut first_bit = '1';
            let mut i = 0;
            while first_bit == '1' {
                let value = self.payload[5 * i..5 * (i + 1)].to_owned();
                first_bit = value.chars().nth(0).unwrap();
                value_str += &value[1..];
                i += 1;
            }

            let value = value_str.chars().rev().enumerate().fold(0, |sum, bit| {
                sum + (u64::from(bit.1.to_digit(2).unwrap()) << bit.0)
            });

            //println!("Value string is {}", value_str);
            //println!("Value is {}", value);

            return value;
        }

        let mut inner_values = vec![];

        for sub_packet in self.sub_packets.iter() {
            inner_values.push(sub_packet.get_value());
        }

        //println!("Inner values are {:?}", inner_values);

        let mut inner_values = inner_values.iter();

        if self.type_id_num == 0 {
            let value = inner_values.sum();

            /*println!(
                "Packet type is {}, summing the above to get {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 1 {
            let value = inner_values.product();

            /*println!(
                "Packet type is {}, multiplying the above to get {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 2 {
            let value = *inner_values.min().unwrap();

            /*println!(
                "Packet type is {}, min from the above is {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 3 {
            let value = *inner_values.max().unwrap();

            /*println!(
                "Packet type is {}, max from the above is {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 5 {
            let mut value = 0;

            if inner_values.next().unwrap() > inner_values.next().unwrap() {
                value = 1;
            }

            /*println!(
                "Packet type is {}, greater than result is {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 6 {
            let mut value = 0;

            if inner_values.next().unwrap() < inner_values.next().unwrap() {
                value = 1;
            }

            /*println!(
                "Packet type is {}, less than result is {}",
                self.type_id_num, value
            );*/

            return value;
        } else if self.type_id_num == 7 {
            let mut value = 0;

            if inner_values.next().unwrap() == inner_values.next().unwrap() {
                value = 1;
            }

            /*println!(
                "Packet type is {}, equal result is {}",
                self.type_id_num, value
            );*/

            return value;
        }

        panic!("Unknown packet type");
    }
}
