use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").expect("should have been able to read the file.");
    let mut weights: Vec<i32> = contents
        .split("\n\n")
        .into_iter()
        .map(|s| tot_weight(s.lines().collect()))
        .collect::<Vec<_>>();
    // match weights.iter().max() {
    //     Some(r) => println!("{}", r),
    //     None => (),
    // }

    weights.sort();
    println!("{}", weights.iter().rev().take(3).sum::<i32>());
}

fn tot_weight(weights: Vec<&str>) -> i32 {
    weights.iter().map(|s| s.parse::<i32>().unwrap()).sum()
}
