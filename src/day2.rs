use std::str::Chars;
use std::fmt;

use get_input::get_input;

const IS_PART1_KEYPAD: bool = false;

#[derive(Copy, Clone)]
enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,  // A, B, C, D for Part 2
    B,
    C,
    D,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Key::One => '1', Key::Two => '2', Key::Three => '3',
            Key::Four => '4', Key::Five => '5', Key::Six => '6',
            Key::Seven => '7', Key::Eight => '8', Key::Nine => '9',
            Key::A => 'A', Key::B => 'B', Key::C => 'C', Key::D => 'D',
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

fn move_one_key(k: Key, d: Direction) -> Key {
    if IS_PART1_KEYPAD {
        match d {
            Direction::Up => match k {
                Key::One | Key::Two | Key::Three => k,
                Key::Four => Key::One, Key::Five => Key::Two, Key::Six => Key::Three,
                Key::Seven => Key::Four, Key::Eight => Key::Five, Key::Nine => Key::Six,
                _ => unreachable!(),
            },
            Direction::Down => match k {
                Key::One => Key::Four, Key::Two => Key::Five, Key::Three => Key::Six,
                Key::Four => Key::Seven, Key::Five => Key::Eight, Key::Six => Key::Nine,
                Key::Seven | Key::Eight | Key::Nine => k,
                _ => unreachable!(),
            },
            Direction::Left => match k {
                Key::One | Key::Four | Key::Seven => k,
                Key::Two => Key::One, Key::Five => Key::Four, Key::Eight => Key::Seven,
                Key::Three => Key::Two, Key::Six => Key::Five, Key::Nine => Key::Eight,
                _ => unreachable!(),
            },
            Direction::Right => match k {
                Key::One => Key::Two, Key::Four => Key::Five, Key::Seven => Key::Eight,
                Key::Two => Key::Three, Key::Five => Key::Six, Key::Eight => Key::Nine,
                Key::Three | Key::Six | Key::Nine => k,
                _ => unreachable!(),
            },
        }
    }
    else {
        match d {
            Direction::Up => match k {
                Key::Five | Key::Two | Key::One | Key::Four | Key::Nine => k,
                Key::Three => Key::One,
                Key::Six => Key::Two, Key::Seven => Key::Three, Key::Eight => Key::Four,
                Key::A => Key::Six, Key::B => Key::Seven, Key::C => Key::Eight,
                Key::D => Key::B,
            },
            Direction::Down => match k {
                Key::One => Key::Three,
                Key::Two => Key::Six, Key::Three => Key::Seven, Key::Four => Key::Eight,
                Key::Six => Key::A, Key::Seven => Key::B, Key::Eight => Key::C,
                Key::B => Key::D,
                Key::Five | Key::A | Key::D | Key::C | Key::Nine => k,
            },
            Direction::Left => match k {
                Key::One | Key::Two | Key::Five | Key::A | Key::D => k,
                Key::Six => Key::Five,
                Key::Three => Key::Two, Key::Seven => Key::Six, Key::B => Key::A,
                Key::Four => Key::Three, Key::Eight => Key::Seven, Key::C => Key::B,
                Key::Nine => Key::Eight,
            },
            Direction::Right => match k {
                Key::Five => Key::Six,
                Key::Two => Key::Three, Key::Six => Key::Seven, Key::A => Key::B,
                Key::Three => Key::Four, Key::Seven => Key::Eight, Key::B => Key::C,
                Key::Eight => Key::Nine,
                Key::One | Key::Four | Key::Nine | Key::C | Key::D => k,
            },
        }
    }
}

fn compute_next_key(k: Key, mut chars: Chars) -> Key {
    let mut current_key = k;
    while let Some(c) = chars.next() {
        let dir = Direction::from_char(c).expect("Invalid input");
        current_key = move_one_key(current_key, dir);
    }
    current_key
}

pub fn solve() {
    let input = get_input("assets/day/2/input");
    let mut current_key = Key::Five;

    for line in input.lines() {
        current_key = compute_next_key(current_key, line.chars());
        print!("{}", current_key);
    }
    println!("");
}
