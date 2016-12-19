use get_input::get_input;

const IS_PART1: bool = false;

pub fn solve() {
    let input = get_input("assets/day/3/input");

    if IS_PART1 {
        let solution: u32 = input.lines().filter_map(|line| {
            let mut words = line.split_whitespace();
            let a = words.next().unwrap().parse::<i32>().unwrap();
            let b = words.next().unwrap().parse::<i32>().unwrap();
            let c = words.next().unwrap().parse::<i32>().unwrap();
            if words.count() != 0 {
                panic!("Too many inputs on one line");
            }

            if a + b > c && b + c > a && a + c > b {
                Some(1u32)
            } else {
                None
            }
        }).sum();

        println!("Part 1: {} possible triangles", solution);
    } else {
        let mut columns = Vec::new();
        let mut triangles = Vec::new();
        for line in input.lines() {
            let mut words = line.split_whitespace();
            columns.push(
                (words.next().unwrap().parse::<i32>().unwrap(),
                words.next().unwrap().parse::<i32>().unwrap(),
                words.next().unwrap().parse::<i32>().unwrap())
            );
            if words.count() != 0 {
                panic!("Too many inputs on one line");
            }

            if columns.len() == 3 {
                let (c1, c2, c3) = columns.pop().unwrap();
                let (b1, b2, b3) = columns.pop().unwrap();
                let (a1, a2, a3) = columns.pop().unwrap();

                triangles.push((a1, b1, c1));
                triangles.push((a2, b2, c2));
                triangles.push((a3, b3, c3));
            }
        }

        let solution: u32 = triangles.iter().filter_map(|&(a, b, c)| {
            if a + b > c && b + c > a && a + c > b {
                Some(1u32)
            } else {
                None
            }
        }).sum();

        println!("Part 2: {} possible triangles", solution);
    }
}
