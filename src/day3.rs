use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for captured in re.captures_iter(input) {
        sum += &captured[1].parse::<u32>().unwrap() * &captured[2].parse().unwrap();
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for captured in re.captures_iter(input) {
        if let Some(_) = captured.get(3) {
            enabled = true;
        } else if let Some(_) = captured.get(4) {
            enabled = false;
        } else if enabled {
            sum += &captured[1].parse::<u32>().unwrap() * &captured[2].parse().unwrap();
        }
    }
    sum
}
