use std::convert::identity;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("should have been able to read the file.");

    let a: usize = input
        .split_terminator("\n")
        .into_iter()
        .map(|a| a.split_once(',').unwrap())
        .map(|c| {
            let ls1 = to_vec(c.0);
            let ls2 = to_vec(c.1);
            // ls1.iter().all(|n| ls2.contains(n)) || ls2.iter().all(|n| ls1.contains(n))
            ls1.iter().any(|n| ls2.contains(n)) || ls2.iter().any(|n| ls1.contains(n))
        })
        .filter(|b| *b)
        .count();
    println!("{}", a);
}

fn to_vec(r: &str) -> Vec<u32> {
    match r.split_once('-') {
        Some(xs) => match (xs.0.parse::<u32>(), xs.1.parse::<u32>()) {
            (Ok(x), Ok(y)) => (x..y + 1).map(identity).collect(),
            _ => unreachable!(),
        },
        None => unreachable!(),
    }
}
