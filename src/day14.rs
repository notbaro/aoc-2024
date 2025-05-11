use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use image::{ImageBuffer, Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

impl Robot {
    pub fn move_once(&mut self) {
        let (mut next_x, mut next_y) = (self.x + self.v_x, self.y + self.v_y);
        if next_x >= COLS {
            next_x -= COLS;
        }
        if next_x < 0 {
            next_x += COLS;
        }
        if next_y >= ROWS {
            next_y -= ROWS;
        }
        if next_y < 0 {
            next_y += ROWS;
        }
        self.x = next_x;
        self.y = next_y;
    }

    pub fn move_n(&mut self, n: usize) {
        for _ in 0..n {
            self.move_once();
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    let mut robots = vec![];
    let regex = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let captured = regex.captures(line).unwrap();
        let x = captured[1].parse::<i64>().unwrap();
        let y = captured[2].parse::<i64>().unwrap();
        let v_x = captured[3].parse::<i64>().unwrap();
        let v_y = captured[4].parse::<i64>().unwrap();
        let robot = Robot { x, y, v_x, v_y };
        robots.push(robot);
    }
    robots
}

pub fn safety_factor(robots: &Vec<Robot>) -> i64 {
    let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);

    let middle_col = (COLS - 1) / 2;
    let middle_row = (ROWS - 1) / 2;

    // Count robots in each quadrant, excluding those on middle lines
    for robot in robots {
        if robot.x < middle_col && robot.y < middle_row {
            top_left += 1;
        } else if robot.x > middle_col && robot.y < middle_row {
            top_right += 1;
        } else if robot.x < middle_col && robot.y > middle_row {
            bottom_left += 1;
        } else if robot.x > middle_col && robot.y > middle_row {
            bottom_right += 1;
        }
        // robots exactly on middle row or middle column are not counted
    }

    let safety_factor = top_left * top_right * bottom_left * bottom_right;
    safety_factor
}

const ROWS: i64 = 103;
const COLS: i64 = 101;

#[aoc(day14, part1)]
pub fn part1(robots: &Vec<Robot>) -> i64 {
    let mut moved_robots = robots.clone();
    for robot in moved_robots.iter_mut() {
        robot.move_n(100);
    }
    safety_factor(&moved_robots)
}

pub fn display_grid(robots: &[Robot]) {
    let mut grid = vec![vec!['.'; COLS as usize]; ROWS as usize];
    
    // Mark robot positions with '#'
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }
    
    // Print the grid
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
    println!("{}", "=".repeat(COLS as usize));
}

pub fn render_grid_as_image(
    robots: &Vec<Robot>,
    time_step: usize,
    file_path_prefix: &str,
) {
    // Create a new black image.
    // Dimensions are cast to u32 as required by ImageBuffer.
    let mut img = ImageBuffer::from_fn(COLS as u32, ROWS as u32, |_, _| {
        Rgb([255u8, 255u8, 255u8]) // White background
    });

    // Mark robot positions (e.g., with black pixels)
    let robot_color = Rgb([0u8, 0u8, 0u8]); // Black

    for robot in robots {
        // Ensure robot coordinates are within bounds (they should be due to wrapping)
        if robot.x >= 0 && robot.x < COLS && robot.y >= 0 && robot.y < ROWS {
            img.put_pixel(robot.x as u32, robot.y as u32, robot_color);
        }
    }

    let file_name = format!("{}_{:05}.png", file_path_prefix, time_step);
    match img.save(&file_name) {
        Ok(_) => println!("Saved frame: {}", file_name),
        Err(e) => eprintln!("Error saving frame {}: {}", file_name, e),
    }
}

#[aoc(day14, part2)]
pub fn part2_visualizer(initial_robots: &Vec<Robot>) -> String { // Or whatever return type AoC expects
    let mut current_robots = initial_robots.clone();

    // Simulate for a certain number of steps, or until the pattern is found.
    // For AoC 2024 Day 14 Part 2, you might need to run for many thousands of steps.
    let max_time_steps = 10000; // Example: render the first 10000 steps

    for t in 0..max_time_steps {
        if t > 0 { // No need to move at t=0
            for robot in current_robots.iter_mut() {
                robot.move_once(); // Assumes COLS and ROWS are accessible by move_once
                                   // or if you use the old constants, pass GRID_WIDTH/GRID_HEIGHT
            }
        }

        // Render the grid at this time step
        // You might choose to render every N steps if rendering every step is too slow
        // or generates too many files.
        if t % 1 == 0 { // Render every frame in this example
            render_grid_as_image(&current_robots, t, "G:/repos/aoc-2024/output_frames/frame");
        }

        // Here, you would also add logic to check if `current_robots`
        // form the "Christmas tree" pattern. The problem is typically
        // vague on how to detect this programmatically, often leading
        // people to visually inspect frames or look for specific geometric
        // properties (e.g., density, symmetry, number of robots in certain rows).
        // if is_christmas_tree(&current_robots) {
        //     println!("Christmas tree found at time step {}!", t);
        //     return t.to_string(); // Example: return the time step
        // }
    }

    "Pattern not found within max_time_steps or visualization complete".to_string()
}