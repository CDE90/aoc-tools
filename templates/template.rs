// Path: {year}/{day}/solution.rs

use std::{fs::File, io::Read};

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

fn read_input() -> String {
    let mut input = String::new();
    let mut file = File::open("./{day}/input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
