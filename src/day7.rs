use get_input::get_input;

const IS_PART1: bool = false;

fn has_abba(s: &&str) -> bool {
    let a = s.chars();
    let b = s.chars().skip(1);
    let c = s.chars().skip(2);
    let d = s.chars().skip(3);
    let mut z = a.zip(d).zip(b.zip(c));
    z.any(|((a,d),(b,c))| {
        a == d && b == c && a != b
    })
}

fn supports_tls(supernets: &Vec<&str>, hypernets: &Vec<&str>) -> bool {
    supernets.into_iter().any(has_abba)
        && !hypernets.into_iter().any(has_abba)
}

fn find_abas(supernets: &Vec<&str>) -> Vec<&str> {
    Vec::new()
}

fn has_matching_bab(aba: &str, hypernets: &Vec<&str>) -> bool {
}

fn supports_ssl(supernets: &Vec<&str>, hypernets: &Vec<&str>) -> bool {
    let abas = find_abas(supernets);
    abas.any(|aba| has_matching_bab(aba, hypernets))
}

fn check_support(ip: &&str) -> bool {
    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();
    let mut left_sides = ip.split('[');
    supernets.push(left_sides.next().unwrap());

    for s in left_sides {
        let mut splits = s.split(']');
        hypernets.push(splits.next().unwrap());
        supernets.push(splits.next().unwrap());
    }

    if IS_PART1 {
        supports_tls(&supernets, &hypernets)
    } else {
        supports_ssl(&supernets, &hypernets)
    }
}

pub fn solve() {
    let input = get_input("assets/day/7/input");

    let soln = input.lines()
                    .filter(check_support)
                    .count();
    println!("{}", soln);
}
