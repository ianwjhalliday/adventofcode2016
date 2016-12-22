use std::collections::HashMap;

use get_input::get_input;

const IS_PART1: bool = false;

fn sector_id(room_name: &str) -> Option<(&str, u32)> {
    let mut name_parts = room_name.split(|c| { c == '-' || c == '[' || c == ']' });
    let _ = name_parts.next_back(); // skip empty string due to delimiter ']' at end of input
    let checksum = name_parts.next_back().unwrap();
    let sector_id = name_parts.next_back().unwrap().parse::<u32>().unwrap();

    let mut histogram = HashMap::new();

    for part in name_parts {
        for c in part.chars() {
            let entry = histogram.entry(c).or_insert(0u32);
            *entry += 1u32;
        }
    }

    let mut v: Vec<(char, u32)> = histogram.iter().map(|(k,v)| (*k, *v)).collect();
    v.sort_by(|&(ch1, count1), &(ch2, count2)| {
        if count1 != count2 {
            count1.cmp(&count2).reverse()
        } else {
            ch1.cmp(&ch2)
        }
    });

    let actual_checksum: String = v.iter().take(checksum.len()).map(|&(c,_)| c).collect();

    if actual_checksum == checksum {
        Some((&room_name[..room_name.rfind('-').unwrap()], sector_id))
    } else {
        None
    }
}

fn decode(s: &str, id: u32) -> String {
    let a = 'a' as u8;
    let rot = (id % 26) as u8; // get rid of full rotations
    let mut decoded = String::with_capacity(s.len());

    for c in s.bytes() {
        if c == '-' as u8 {
            decoded.push(' ');
        } else {
            decoded.push(((c - a + rot) % 26 + a) as char);
        }
    }

    decoded
}

pub fn solve() {
    let input = get_input("assets/day/4/input");
    let real_rooms = input.lines().filter_map(sector_id);

    if IS_PART1 {
        let solution: u32 = real_rooms.map(|(_,c)| c).sum();
        println!("Sector IDs sum is {}", solution);
    } else {
        for (encoded_name, id) in real_rooms {
            let decoded_name = decode(encoded_name, id);
            println!("{}: {}", id, decoded_name);
        }
    }
}
