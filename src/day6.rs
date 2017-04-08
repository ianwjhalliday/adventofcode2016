use std::collections::HashMap;

use get_input::get_input;

const IS_PART1: bool = false;

pub fn solve() {
    let input = get_input("assets/day/6/input");
    let msg_len = input.lines().next().unwrap().chars().count();

    println!("{}", msg_len);

    let mut correct_msg = Vec::with_capacity(msg_len);
    correct_msg.resize(msg_len, ' ');

    let mut max = Vec::with_capacity(msg_len);
    max.resize(msg_len, 0);

    let mut histos = Vec::with_capacity(msg_len);
    for _ in 0..msg_len {
        histos.push(HashMap::new());
    }

    for msg in input.lines() {
        for (i, c) in msg.chars().enumerate() {
            let entry = histos[i].entry(c).or_insert(0);
            *entry += 1;

            if IS_PART1 && *entry > max[i] {
                max[i] = *entry;
                correct_msg[i] = c;
            }
        }
    };

    if IS_PART1 {
        let message: String = correct_msg.into_iter().collect();
        println!("{}", message);
    } else {
        let message: String =
            histos.into_iter()
                  .map(|h| h.iter().min_by(|a,b| a.1.cmp(b.1)).unwrap().0.clone())
                  .collect();
        println!("{}", message);
    }
}
