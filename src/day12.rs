use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn grid_generator(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    grid
}

#[derive(Debug)]
struct Region {
    _plant: char,
    plots: HashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self, rows: usize, cols: usize) -> usize {
        let mut perimeter = 0;
        for (plot_r, plot_c) in &self.plots {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let neighbor_r = *plot_r as i32 + dr;
                let neighbor_c = *plot_c as i32 + dc;

                if !(neighbor_r >= 0
                    && neighbor_r < rows as i32
                    && neighbor_c >= 0
                    && neighbor_c < cols as i32
                    && self
                        .plots
                        .contains(&(neighbor_r as usize, neighbor_c as usize)))
                {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn sides(&self) -> usize {
        let mut corners = 0;
        let top_left = [(-1, 0), (0, -1), (-1, -1)];
        let top_right = [(-1, 0), (0, 1), (-1, 1)];
        let bottom_left = [(1, 0), (0, -1), (1, -1)];
        let bottom_right = [(1, 0), (0, 1), (1, 1)];
        let orientation = [top_left, top_right, bottom_left, bottom_right];
        for (plot_r, plot_c) in &self.plots {
            for [(dr1, dc1), (dr2, dc2), (dr3, dc3)] in orientation {
                let adj1 = (
                    (*plot_r as i32 + dr1) as usize,
                    (*plot_c as i32 + dc1) as usize,
                );

                let adj2 = (
                    (*plot_r as i32 + dr2) as usize,
                    (*plot_c as i32 + dc2) as usize,
                );
                let diag = (
                    (*plot_r as i32 + dr3) as usize,
                    (*plot_c as i32 + dc3) as usize,
                );
                let is_convex = !self.plots.contains(&adj1) && !self.plots.contains(&adj2);
                let is_concave = self.plots.contains(&adj1) && self.plots.contains(&adj2) && !self.plots.contains(&diag);

                if is_convex || is_concave {
                    corners += 1;
                }
            }
        }
        corners
    }

    fn price(&self, rows: usize, cols: usize) -> usize {
        self.area() * self.perimeter(rows, cols)
    }
}

fn get_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = vec![];
    let mut visited = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if !visited.contains(&(r, c)) {
                let plant = grid[r][c];
                let mut plots = HashSet::new();
                let mut queue = VecDeque::new();

                visited.insert((r, c));
                queue.push_back((r, c));
                plots.insert((r, c));

                while let Some((curr_r, curr_c)) = queue.pop_front() {
                    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let next_r = curr_r as i32 + dr;
                        let next_c = curr_c as i32 + dc;

                        if next_r >= 0
                            && next_r < grid.len() as i32
                            && next_c >= 0
                            && next_c < grid[0].len() as i32
                        {
                            let nr = next_r as usize;
                            let nc = next_c as usize;
                            if grid[nr][nc] == plant && !visited.contains(&(nr, nc)) {
                                visited.insert((nr, nc));
                                plots.insert((nr, nc));
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
                let new_region = Region {
                    _plant: plant,
                    plots,
                };
                regions.push(new_region);
            }
        }
    }
    regions
}

#[aoc(day12, part1)]
pub fn part1(grid: &Vec<Vec<char>>) -> usize {
    let regions = get_regions(grid);
    regions
        .iter()
        .map(|r| r.price(grid.len(), grid[0].len()))
        .sum::<usize>()
}

#[aoc(day12, part2)]
pub fn part2(grid: &Vec<Vec<char>>) -> usize {
    let regions = get_regions(grid);
    let mut p = 0;
    for r in regions {
        let s = r.sides();
        println!("{}", r._plant);
        dbg!(s);
        p += s* r.area();
    }
    p
}
