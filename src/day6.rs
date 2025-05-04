use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    grid
}
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    Up,    // (0, 1)
    Right, // (1, 0)
    Down,  // (0, -1)
    Left,  // (-1, 0)
}

impl Direction {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn find_start_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!()
}

#[aoc(day6, part1)]
pub fn part1(grid: &Vec<Vec<char>>) -> u32 {
    let mut grid = grid.clone();
    let mut pos = find_start_pos(&grid);

    const BARRIER: char = '#';
    const UNVISITED: char = '.';
    const VISITED: char = 'X';

    let mut dir = Direction::Up;
    let mut in_bound = true;
    let mut visited_count = 0;

    grid[pos.0][pos.1] = VISITED;
    visited_count += 1;
    while in_bound {
        let next_pos = (
            (pos.0 as i32 + dir.offset().0) as usize,
            (pos.1 as i32 + dir.offset().1) as usize,
        );
        in_bound = next_pos.0 < grid.len() && next_pos.1 < grid[0].len();
        if !in_bound {
            break;
        }
        match grid[next_pos.0][next_pos.1] {
            BARRIER => dir = dir.next(),
            UNVISITED => {
                grid[next_pos.0][next_pos.1] = VISITED;
                pos = next_pos;
                visited_count += 1;
            }
            VISITED => pos = next_pos,
            _ => panic!(),
        }
    }
    visited_count
}

pub fn has_loop(grid: Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    const BARRIER: char = '#';

    let mut dir = Direction::Up;
    let mut in_bound = true;
    let mut has_loop = false;
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut pos = start_pos;

    while in_bound {
        let current_state = (pos, dir);

        if visited.contains(&current_state) {
            has_loop = true;
            break;
        }
        let next_pos = (
            (pos.0 as i32 + dir.offset().0) as usize,
            (pos.1 as i32 + dir.offset().1) as usize,
        );
        in_bound = next_pos.0 < grid.len() && next_pos.1 < grid[0].len();
        if !in_bound {
            break;
        }

        visited.insert(current_state);
        if grid[next_pos.0][next_pos.1] == BARRIER {
            dir = dir.next();
        } else {
            pos = next_pos;
        }
    }
    has_loop
}

#[aoc(day6, part2)]
pub fn part2(grid: &Vec<Vec<char>>) -> u32 {
    let mut loop_count = 0;
    let start_pos = find_start_pos(grid);
    let mut grid = grid.clone();
    grid[start_pos.0][start_pos.1] = '.';

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if (row, col) == start_pos {
                continue;
            }

            if grid[row][col] == '#' {
                continue;
            } else {
                let mut grid_clone = grid.clone();
                grid_clone[row][col] = '#';
                if has_loop(grid_clone, start_pos) {
                    loop_count += 1;
                }
            }
        }
    }

    loop_count
}
