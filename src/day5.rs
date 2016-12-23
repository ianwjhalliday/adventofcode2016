use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::{BinaryHeap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;

// Quick timing results on my laptop
//
// Elapsed |        |        |
// Time    | Single | Multi  |
// --------|--------|--------|
// Part 1  |  3.5 s |  2.5 s |
// Part 2  | 11.9 s |  7.7 s |
// --------|--------|--------|

const IS_PART1: bool = false;
const IS_CONCURRENT: bool = true;
const NUM_CPUS: usize = 4; // assuming my machine

pub fn solve() {
    if !IS_CONCURRENT {
        return solve_no_concurrency();
    }

    let input = "cxdnnyjw";

    let nonces = Arc::new(Mutex::new(0..));
    let results = Arc::new(Mutex::new(BinaryHeap::new()));
    let index_set = Arc::new(Mutex::new(HashSet::new()));

    let mut threads = Vec::with_capacity(NUM_CPUS);

    for _ in 0..NUM_CPUS {
        let nonces = nonces.clone();
        let results = results.clone();
        let index_set = index_set.clone();

        threads.push(thread::spawn(move || {
            let mut sh = Md5::new();
            sh.input_str(input);

            while results.lock().unwrap().len() < 8 {
                let nonce = nonces.lock().unwrap().next().unwrap();
                let mut sh = sh.clone();

                sh.input_str(&nonce.to_string());
                let hash = sh.result_str();

                if hash.starts_with("00000") {
                    let ch = hash.chars().nth(5).unwrap();
                    if IS_PART1 {
                        println!("{}: {}", nonce, ch);
                        results.lock().unwrap().push(Result {
                            priority: nonce,
                            ch: ch,
                        });
                    } else if ch >= '0' && ch <= '7' {
                        let i = (ch as u8 - b'0') as i32;
                        let mut index_set = index_set.lock().unwrap();
                        if !index_set.contains(&i) {
                            let ch = hash.chars().nth(6).unwrap();
                            println!("{}: {} {}", nonce, i, ch);
                            index_set.insert(i);
                            results.lock().unwrap().push(Result {
                                priority: i,
                                ch: ch,
                            });
                        } else {
                            println!("{}: {} SKIPPED", nonce, i);
                        }
                    }
                }
            }
        }));
    }

    for t in threads {
        t.join().ok();
    }

    // take(8) in case one thread slipped in a 9th result
    // before the 8th result was computed. The priority
    // queue ensures we have the correct 8 up front.
    let password = results.lock().unwrap().clone()
        .into_sorted_vec().iter()
        .take(8).map(|r| r.ch).collect::<String>();
    println!("Password cracked!\npassword: {}", password);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Result {
    priority: i32,
    ch: char,
}

impl Ord for Result {
    fn cmp(&self, other: &Result) -> ::std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

// BinaryHeap requires both Ord and PartialOrd
impl PartialOrd for Result {
    fn partial_cmp(&self, other: &Result) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// My original solution before diving into std::sync
// for some concurrency fun
pub fn solve_no_concurrency() {
    let input = "cxdnnyjw";

    let mut sh = Md5::new();
    sh.input_str(input);

    if IS_PART1 {
        let password = (0..).filter_map(|nonce| {
            let mut sh = sh.clone();
            sh.input_str(nonce.to_string().as_str());
            let hash = sh.result_str();
            if hash.starts_with("00000") {
                let ch = hash.chars().nth(5).unwrap();
                println!("{}: {}", nonce, ch);
                Some(ch)
            } else {
                None
            }
        }).take(8).collect::<String>();

        println!("{}", password);
    } else {
        let mut password: Vec<Option<char>> = vec![None; 8];

        for nonce in 0.. {
            let mut sh = sh.clone();
            sh.input_str(nonce.to_string().as_str());
            let hash = sh.result_str();
            if hash.starts_with("00000") {
                let i = hash.chars().nth(5).unwrap();
                match i {
                    '0'...'7' => {
                        let i = (i as u8 - b'0') as usize;
                        if password[i].is_none() {
                            password[i] = Some(
                                hash.chars().nth(6).unwrap());
                        }
                    },
                    _ => ()
                }
                if !password.contains(&None) {
                    break;
                }
            }
        }

        let password = password.iter().map(|c| c.unwrap()).collect::<String>();
        println!("{}", password);
    }

    // iterator to produce key+nonce enumerated values where nonce starts at 0
    // and is incremented forever
    //
    // in parallel (once for each CPU), pull value from iterator and compute
    // md5 hash, and if first five characters are each '0', place hash and
    // nonce into priority queue, prioritized on nonce value, lower comes
    // first.
    //
    // once eight values are in queue, print out sixth character in each hash
    // in order. This is the password.
    //
    // Terminate
    //
    //
    // Or...
    // can we do a take(8).collect().to_string() on a parallel iterator?
    // No, I cannot figure out how to do this quickly enough.
}
