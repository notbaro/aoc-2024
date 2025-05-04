use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn grid_generator(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    grid
}

pub fn find_x(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut x_pos: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                x_pos.push((row, col));
            }
        }
    }
    x_pos
}

pub fn find_a(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut x_pos: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'A' {
                x_pos.push((row, col));
            }
        }
    }
    x_pos
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,        // (0, 1)
    UpRight,   // (1, 1)
    Right,     // (1, 0)
    DownRight, // (1, -1)
    Down,      // (0, -1)
    DownLeft,  // (-1, -1)
    Left,      // (-1, 0)
    UpLeft,    // (-1, 1)
}

impl Direction {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::UpRight => (1, 1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, -1),
            Direction::Down => (0, -1),
            Direction::DownLeft => (-1, -1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, 1),
        }
    }

    pub fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }
}

pub fn is_xmas(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: Direction) -> bool {
    let offset = dir.offset();
    let access = || -> Result<(char, char, char), Box<dyn Error>> {
        // Convert position to i32 for calculation
        let row = pos.0 as i32;
        let col = pos.1 as i32;

        // Calculate new positions
        let row2 = row + offset.0;
        let col2 = col + offset.1;
        let row3 = row + offset.0 * 2;
        let col3 = col + offset.1 * 2;
        let row4 = row + offset.0 * 3;
        let col4 = col + offset.1 * 3;

        // Check bounds
        if row2 < 0
            || col2 < 0
            || row3 < 0
            || col3 < 0
            || row4 < 0
            || col4 < 0
            || row2 >= grid.len() as i32
            || col2 >= grid[0].len() as i32
            || row3 >= grid.len() as i32
            || col3 >= grid[0].len() as i32
            || row4 >= grid.len() as i32
            || col4 >= grid[0].len() as i32
        {
            return Err("Out of bounds".into());
        }

        // Convert back to usize for indexing
        let second = grid[row2 as usize][col2 as usize];
        let third = grid[row3 as usize][col3 as usize];
        let fourth = grid[row4 as usize][col4 as usize];

        Ok((second, third, fourth))
    };
    if let Ok(('M', 'A', 'S')) = access() {
        return true;
    } else {
        false
    }
}

pub fn is_x_mas2(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    let (row, col) = (pos.0 as i32, pos.1 as i32);
    let (row_tl, col_tl) = (row - 1, col - 1);
    let (row_tr, col_tr) = (row - 1, col + 1);
    let (row_bl, col_bl) = (row + 1, col - 1);
    let (row_br, col_br) = (row + 1, col + 1);

    // Check if the positions are valid (within grid bounds)
    if row_tl >= 0
        && col_tl >= 0
        && row_tr >= 0
        && col_tr >= 0
        && row_bl >= 0
        && col_bl >= 0
        && row_br >= 0
        && col_br >= 0
        && row_tl < grid.len() as i32
        && col_tl < grid[0].len() as i32
        && row_tr < grid.len() as i32
        && col_tr < grid[0].len() as i32
        && row_bl < grid.len() as i32
        && col_bl < grid[0].len() as i32
        && row_br < grid.len() as i32
        && col_br < grid[0].len() as i32
    {
        // Now it's safe to access these positions
        let char_tl = grid[row_tl as usize][col_tl as usize];
        let char_tr = grid[row_tr as usize][col_tr as usize];
        let char_bl = grid[row_bl as usize][col_bl as usize];
        let char_br = grid[row_br as usize][col_br as usize];

        // Add your pattern checking logic here
        let first_pair =
            ((char_tl == 'M') && (char_br == 'S')) || ((char_tl == 'S') && (char_br == 'M'));
        let second_pair =
            ((char_tr == 'M') && (char_bl == 'S')) || ((char_tr == 'S') && (char_bl == 'M'));
        if first_pair && second_pair { 1 } else { 0 }
    } else {
        // Out of bounds, handle appropriately
        0
    }
}

pub fn count_xmas(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    let mut count = 0;
    for direction in Direction::all() {
        if is_xmas(grid, pos, direction) {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part1)]
pub fn part1(grid: &Vec<Vec<char>>) -> u32 {
    let x_pos = find_x(grid);
    let mut count = 0;
    for pos in x_pos {
        count += count_xmas(grid, pos);
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(grid: &Vec<Vec<char>>) -> u32 {
    let a_pos = find_a(grid);
    let mut count = 0;
    for pos in a_pos {
        count += is_x_mas2(grid, pos);
    }
    count
}
