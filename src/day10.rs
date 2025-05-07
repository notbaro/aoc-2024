use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    let mut grid = vec![];
    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    grid
}

pub fn find_trailheads(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut pos = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == 0 {
                pos.push((i, j));
            }
        }
    }
    pos
}

pub fn reachable_summits_from_trailhead(
    grid: &Vec<Vec<u8>>,
    trailhead_pos: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut reachable_summits = HashSet::new();
    //BFS
    let mut queue = vec![trailhead_pos];
    let mut visited = HashSet::new();
    visited.insert(trailhead_pos);

    while !queue.is_empty() {
        let (r, c) = queue.remove(0);
        let height = grid[r][c];

        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (next_r, next_c) = (r as i32 + dr as i32, c as i32 + dc as i32);
            let (next_r, next_c) = (next_r as usize, next_c as usize);

            if next_r < grid.len() && next_c < grid[0].len() {
                let next_height = grid[next_r][next_c];
                if next_height == height + 1 && !visited.contains(&(next_r, next_c)) {
                    visited.insert((next_r, next_c));
                    if next_height == 9 {
                        reachable_summits.insert((next_r, next_c));
                    } else {
                        queue.push((next_r, next_c));
                    }
                }
            }
        }
    }
    reachable_summits
}

#[aoc(day10, part1)]
pub fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let mut total_score = 0;
    for trailhead in find_trailheads(grid) {
        total_score += reachable_summits_from_trailhead(grid, trailhead).len();
    }
    total_score
}

fn count_paths(
    grid: &Vec<Vec<u8>>,
    current_pos: (usize, usize),
    memo_cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    //memo
    if let Some(&cached) = memo_cache.get(&current_pos) {
        return cached;
    }
    //base case
    let height = grid[current_pos.0][current_pos.1];
    if height == 9 {
        memo_cache.insert(current_pos, 1);
        return 1;
    }

    let mut num_paths = 0;
    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (next_r, next_c) = (current_pos.0 as i32 + dr as i32, current_pos.1 as i32 + dc as i32);
        let (next_r, next_c) = (next_r as usize, next_c as usize);

        if next_r < grid.len() && next_c < grid[0].len() && grid[next_r][next_c] == height + 1 {
            num_paths += count_paths(grid, (next_r, next_c), memo_cache);
        }
    }
    memo_cache.insert(current_pos, num_paths);
    num_paths
}

#[aoc(day10, part2)]
pub fn part2(grid: &Vec<Vec<u8>>) -> usize {
    let mut total_score = 0;
    let mut memo_cache = HashMap::new();
    for trailhead in find_trailheads(grid) {
        total_score += count_paths(grid, trailhead, &mut memo_cache);
    }
    total_score
}
