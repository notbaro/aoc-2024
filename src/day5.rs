use std::{cmp::Ordering, collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    let mut ordering = HashSet::new();
    let mut numbers = Vec::new();
    let mut is_first_section = true;
    for line in input.lines() {
        if line.is_empty() {
            is_first_section = false;
        } else if is_first_section {
            let (left, right) = line.split_once('|').unwrap();
            ordering.insert((left.parse().unwrap(), right.parse().unwrap()));
        } else {
            numbers.push(line.split(',').map(|x| x.parse::<u8>().unwrap()).collect());
        }
    }
    (ordering, numbers)
}

//returns 0 if not valid
pub fn valid(number: &Vec<u8>, ordering: &HashSet<(u8, u8)>) -> u8 {
    for i in 0..number.len() - 1 {
        for j in i + 1..number.len() {
            if ordering.contains(&(number[j], number[i])) {
                return 0;
            }
        }
    }
    number[number.len() / 2]
}
#[aoc(day5, part1)]
pub fn part1((ordering, numbers): &(HashSet<(u8, u8)>, Vec<Vec<u8>>)) -> u32 {
    let mut sum = 0;
    for number in numbers {
        sum += valid(number, ordering) as u32;
    }
    sum
}

#[aoc(day5, part2)]
pub fn part2((ordering, numbers): &(HashSet<(u8, u8)>, Vec<Vec<u8>>)) -> u32 {
    let mut sum = 0;
    for number in numbers {
        if valid(number, ordering) == 0 {
            let mut number = number.clone();
            number.sort_by(|a, b| {
                if ordering.contains(&(*a, *b)) {
                    Ordering::Less
                } else if ordering.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            sum += number[number.len() / 2] as u32;
        }
    }
    sum
}
