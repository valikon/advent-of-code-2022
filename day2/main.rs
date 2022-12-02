use std::fmt;
use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn mk(s: &str) -> Shape {
        match s {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Shape::Rock => write!(f, "Rock"),
            Shape::Paper => write!(f, "Paper"),
            Shape::Scissors => write!(f, "Scissors"),
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn mk(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./input").expect("should have been able to read the file.");

    let score: u32 = contents
        .split_terminator("\n")
        .into_iter()
        .map(|s| s.split_at(1))
        // .map(|xs| (Shape::mk(xs.0.trim()), Shape::mk(xs.1.trim())))
        // .map(|ys| get_points(ys))
        .map(|xs| (Shape::mk(xs.0.trim()), Outcome::mk(xs.1.trim())))
        .map(|ys| get_modified_points(ys))
        .sum();

    println!("{}", score);
}

fn get_points(shapes: (Shape, Shape)) -> u32 {
    match shapes {
        // (others, mine)
        (Shape::Rock, Shape::Rock) => 3 + 1,
        (Shape::Rock, Shape::Paper) => 6 + 2,
        (Shape::Rock, Shape::Scissors) => 0 + 3,

        (Shape::Paper, Shape::Rock) => 0 + 1,
        (Shape::Paper, Shape::Paper) => 3 + 2,
        (Shape::Paper, Shape::Scissors) => 6 + 3,

        (Shape::Scissors, Shape::Rock) => 6 + 1,
        (Shape::Scissors, Shape::Paper) => 0 + 2,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    }
}

fn get_modified_points(shape_outcome: (Shape, Outcome)) -> u32 {
    match shape_outcome {
        (Shape::Rock, Outcome::Win) => 6 + 2,
        (Shape::Rock, Outcome::Draw) => 3 + 1,
        (Shape::Rock, Outcome::Loss) => 0 + 3,

        (Shape::Paper, Outcome::Win) => 6 + 3,
        (Shape::Paper, Outcome::Draw) => 3 + 2,
        (Shape::Paper, Outcome::Loss) => 0 + 1,

        (Shape::Scissors, Outcome::Win) => 6 + 1,
        (Shape::Scissors, Outcome::Draw) => 3 + 3,
        (Shape::Scissors, Outcome::Loss) => 0 + 2,
    }
}
