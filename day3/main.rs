use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("should have been able to read the file.");

    // let sum_value: u32 = input
    //     .split_terminator("\n")
    //     .into_iter()
    //     .map(|s| s.split_at(s.len() / 2))
    //     .map(|(a, b)| calculate_priority(a, &[b]))
    //     .sum::<u32>();

    let sum_value: u32 = input
        .split_terminator("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| calculate_priority(group[0], &group[1..=2]))
        .sum();

    println!("{}", sum_value);
}

fn calculate_priority(a: &str, rest: &[&str]) -> u32 {
    let mut a_chars = a.chars().collect::<Vec<char>>();
    a_chars.sort();
    a_chars.dedup();
    a_chars
        .iter()
        .filter(|a_char| rest.iter().all(|r| r.contains(**a_char)))
        .map(|c| get_char_value(*c))
        .sum()
}

fn get_char_value(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}
