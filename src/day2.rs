use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut lines = input.lines();
    let mut data: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        data.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }
    data
}

fn is_safe(report: &Vec<u32>) -> bool {
    let mut diff = 0;
    for window in report.windows(2) {
        if let &[first, second] = window {
            let new_diff = second as i32 - first as i32;
            if diff * new_diff < 0 || new_diff > 3 || new_diff < -3 || new_diff == 0 {
                return false;
            }
            diff = new_diff
        }
    }
    true
}

fn is_somewhat_safe(report: &Vec<u32>) -> bool {
    let mut v: Vec<Vec<u32>> = Vec::with_capacity(report.len());
    for i in 0..report.len() {
        let cut_vec = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .cloned()
            .collect::<Vec<u32>>();
        v.push(cut_vec);
    }
    for small_vector in v {
        if is_safe(&small_vector) {
            return true;
        }
    }
    false
}

#[aoc(day2, part1)]
pub fn part1(data: &Vec<Vec<u32>>) -> u32 {
    let mut safe_count = 0u32;
    for report in data {
        if is_safe(report) || is_somewhat_safe(report) {
            safe_count += 1
        }
    }
    safe_count
}
