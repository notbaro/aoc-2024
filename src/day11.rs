use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<u128, u128> {
    let mut numbers = HashMap::new();
    for num in input.split_ascii_whitespace() {
        numbers
            .entry(num.parse().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    numbers
}

const ODD_LENGTH_MULTIPLIER: u128 = 2024;

fn blink_once(stones: &mut HashMap<u128, u128>) {
    let mut next_stones: HashMap<u128, u128> = HashMap::new();
    for (&stone, &count) in stones.iter() {
        if stone == 0 {
            *next_stones.entry(1).or_insert(0) += count;
        } else if stone.to_string().len() % 2 == 0 {
            let mut left = stone.to_string();
            let right = left.split_off(left.len() / 2);
            let (left_val, right_val) = (
                left.parse::<u128>().unwrap(),
                right.parse::<u128>().unwrap(),
            );
            *next_stones.entry(left_val).or_insert(0) += count;
            *next_stones.entry(right_val).or_insert(0) += count;
        } else {
            if let Some(new_stone_val) = stone.checked_mul(ODD_LENGTH_MULTIPLIER) {
                *next_stones.entry(new_stone_val).or_insert(0) += count;
            } else {
                panic!("Overflow occurred");
            }
        }
    }
    *stones = next_stones;
}

#[aoc(day11, part1)]
pub fn part1(numbers: &HashMap<u128, u128>) -> usize {
    let mut stones = numbers.clone();
    for _ in 0..25 {
        blink_once(&mut stones);
    }
    stones.values().map(|&v| v as usize).sum()
}

#[aoc(day11, part2)]
pub fn part2(numbers: &HashMap<u128, u128>) -> u128 {
    let mut stones = numbers.clone();
    for _ in 0..75 {
        blink_once(&mut stones);
    }
    stones.values().map(|&v| v as u128).sum::<u128>()
}
