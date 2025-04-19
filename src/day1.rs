use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {


    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        first_vec.push(nums[0]);
        second_vec.push(nums[1]);
    }
     
    //sort the vectors
    first_vec.sort();
    second_vec.sort();
    
    (first_vec, second_vec)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<i32>, Vec<i32>)) -> u32 {
    let (first_vec, second_vec) = input;
    let mut sum = 0;

    for i in 0..first_vec.len() {
        sum += (first_vec[i] - second_vec[i]).abs();
    }

    sum as u32
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<i32>, Vec<i32>)) -> u32 {
    let (first_vec, second_vec) = input;
    let mut sum = 0;
    for i in 0..first_vec.len() {
        let mut count = 0;
        for j in 0..second_vec.len() {
            if first_vec[i] == second_vec[j] {
                count += 1;
            }
        }
        sum += first_vec[i] * count;
    }
    sum as u32
}