use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(usize, usize);

pub fn scan_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Coord>> {
    let mut antennas_coordinates = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            if element == '.' {
                continue;
            } else {
                let coord = Coord(i, j);
                antennas_coordinates
                    .entry(element)
                    .and_modify(|coor: &mut Vec<Coord>| {
                        coor.push(coord);
                    })
                    .or_insert(vec![coord]);
            }
        }
    }
    antennas_coordinates
}

pub fn in_bounds(coord: Coord, max_row: usize, max_col: usize) -> bool {
    coord.0 < max_row && coord.1 < max_col
}

pub fn antinodes_from_pair(
    first_coord: &Coord,
    second_coord: &Coord,
    max_row: usize,
    max_col: usize,
) -> HashSet<Coord> {
    let f = |a, b| 2 * a - b;
    let antinodes_1 = Coord(
        f(first_coord.0, second_coord.0),
        f(first_coord.1, second_coord.1),
    );
    let antinodes_2 = Coord(
        f(second_coord.0, first_coord.0),
        f(second_coord.1, first_coord.1),
    );

    let mut valid_antinodes = HashSet::new();
    if in_bounds(antinodes_1, max_row, max_col) {
        valid_antinodes.insert(antinodes_1);
    }
    if in_bounds(antinodes_2, max_row, max_col) {
        valid_antinodes.insert(antinodes_2);
    }
    valid_antinodes
}

pub fn find_antinodes(
    grid: &Vec<Vec<char>>,
    antennas: &Vec<Coord>,
    from_pair_fn: fn(
        first_coord: &Coord,
        second_coord: &Coord,
        max_row: usize,
        max_col: usize,
    ) -> HashSet<Coord>,
) -> HashSet<Coord> {
    let pairs_iter = antennas.iter().tuple_combinations::<(_, _)>();
    let mut antinodes_coord_for_freq = HashSet::new();
    for pair in pairs_iter {
        let found_antinodes = from_pair_fn(pair.0, pair.1, grid.len(), grid[0].len());
        antinodes_coord_for_freq.extend(found_antinodes.iter());
    }
    antinodes_coord_for_freq
}

#[aoc(day8, part1)]
pub fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut antinodes_coordinates = HashSet::new();
    let antennas_coordinates = scan_antennas(grid);
    for (_freq, antennas) in &antennas_coordinates {
        let antinodes_coord_for_freq = find_antinodes(grid, antennas, antinodes_from_pair);
        for coord in antinodes_coord_for_freq {
            antinodes_coordinates.insert(coord);
        }
    }
    antinodes_coordinates.len()
}

pub fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn antinodes_from_pair_part2(
    first_coord: &Coord,
    second_coord: &Coord,
    max_row: usize,
    max_col: usize,
) -> HashSet<Coord> {
    let mut valid_antinodes = HashSet::new();
    let (row_diff, col_diff) = (
        first_coord.0 as i32 - second_coord.0 as i32,
        first_coord.1 as i32 - second_coord.1 as i32,
    );
    let g = gcd(row_diff, col_diff);
    let dr = row_diff / g;
    let dc = col_diff / g;
    //first side
    let mut pos = first_coord.clone();
    loop {
        valid_antinodes.insert(pos);
        let next_row = (pos.0 as i32 + dr) as usize;
        let next_col = (pos.1 as i32 + dc) as usize;
        if in_bounds(Coord(next_row, next_col), max_row, max_col) {
            pos = Coord(next_row, next_col);
        } else {
            break;
        }
    }
    //second side
    pos = *first_coord;
    loop {
        valid_antinodes.insert(pos);
        let next_row = (pos.0 as i32 - dr) as usize;
        let next_col = (pos.1 as i32 - dc) as usize;
        if in_bounds(Coord(next_row, next_col), max_row, max_col) {
            pos = Coord(next_row, next_col);
        } else {
            break;
        }
    }
    valid_antinodes
}

pub fn dbg_prt(grid: &Vec<Vec<char>>) {
    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}
#[aoc(day8, part2)]
pub fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut antinodes_coordinates = HashSet::new();
    let antennas_coordinates = scan_antennas(grid);
    for (_freq, antennas) in &antennas_coordinates {
        let antinodes_coord_for_freq = find_antinodes(grid, antennas, antinodes_from_pair_part2);
        for coord in antinodes_coord_for_freq {
            antinodes_coordinates.insert(coord);
        }
    }

    dbg_prt(grid);
    println!("----------------------------------------------");
    let mut grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if antinodes_coordinates.contains(&Coord(i, j)) && grid[i][j] == '.' {
                grid[i][j] = '#'
            }
        }
    }
    dbg_prt(&grid);

    antinodes_coordinates.len()
}
