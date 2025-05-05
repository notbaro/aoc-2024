use std::iter::repeat;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(u128, Vec<u128>)> {
    let mut equations: Vec<(u128, Vec<u128>)> = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(':').unwrap();
        let left = left.parse::<u128>().unwrap();
        let right = right
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect();
        equations.push((left, right));
    }
    equations
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Op {
    Plus,
    Multiply,
    Concatenate,
}

pub fn generate_operators_combination_part1(size: usize) -> Vec<Vec<Op>> {
    let choices = [Op::Plus, Op::Multiply];
    repeat(choices.iter())
        .take(size)
        .multi_cartesian_product()
        .map(|x| x.into_iter().cloned().collect())
        .collect()
}

pub fn generate_operators_combination_part2(size: usize) -> Vec<Vec<Op>> {
    let choices = [Op::Plus, Op::Multiply, Op::Concatenate];
    repeat(choices.iter())
        .take(size)
        .multi_cartesian_product()
        .map(|x| x.into_iter().cloned().collect())
        .collect()
}

pub fn concat(first: u128, second: u128) -> u128 {
    let first_str = first.to_string();
    let second_str = second.to_string();
    let concat_str = first_str + &second_str;
    concat_str.parse::<u128>().unwrap()
}

pub fn eval(num: &[u128], operators: &[Op]) -> u128 {
    let mut result = num[0];
    for i in 1..num.len() {
        result = match operators[i - 1] {
            Op::Plus => result + num[i],
            Op::Multiply => result * num[i],
            Op::Concatenate => concat(result, num[i]),
        }
    }
    result
}

pub fn calibrate_result(
    (lhs, rhs): &(u128, Vec<u128>),
    generate_comb_fn: fn(usize) -> Vec<Vec<Op>>,
) -> u128 {
    for operations in generate_comb_fn(rhs.len() - 1) {
        if *lhs == eval(rhs, &operations) {
            return *lhs;
        }
    }
    0
}
#[aoc(day7, part1)]
pub fn part1(equations: &Vec<(u128, Vec<u128>)>) -> u128 {
    equations
        .iter()
        .map(|e| calibrate_result(e, generate_operators_combination_part1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(equations: &Vec<(u128, Vec<u128>)>) -> u128 {
    equations
        .iter()
        .map(|e| calibrate_result(e, generate_operators_combination_part2))
        .sum()
}
