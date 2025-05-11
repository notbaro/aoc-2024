use std::i128;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Prize {
    button_a: (i128, i128),
    button_b: (i128, i128),
    coords: (i128, i128),
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Prize> {
    let mut prizes = vec![];
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let lines: Vec<_> = input.lines().collect();
    for chunk in lines.chunks(4) {
        let text = chunk.join("\n");
        if let Some(caps) = regex.captures(&text) {
            let button_a_x = caps[1].parse::<i128>().unwrap();
            let button_a_y = caps[2].parse::<i128>().unwrap();
            let button_b_x = caps[3].parse::<i128>().unwrap();
            let button_b_y = caps[4].parse::<i128>().unwrap();
            let prize_x = caps[5].parse::<i128>().unwrap();
            let prize_y = caps[6].parse::<i128>().unwrap();

            prizes.push(Prize {
                button_a: (button_a_x, button_a_y),
                button_b: (button_b_x, button_b_y),
                coords: (prize_x, prize_y),
            });
        }
    }
    prizes
}

pub fn solve(p: &Prize) -> Option<i128> {
    /*
    x1*a + x2*b = p1
    y1*a + y2*b = p2
    A: (x1, y1)
    B: (x2, y2)
    P: (p1, p2)
     */

    let (x1, x2) = (p.button_a.0, p.button_b.0);
    let (y1, y2) = (p.button_a.1, p.button_b.1);
    let (p1, p2) = (p.coords.0, p.coords.1);

    let d = (x1 * y2) as i128 - (y1 * x2) as i128;
    let da = (p1 * y2) as i128 - (p2 * x2) as i128;
    let db = (x1 * p2) as i128 - (y1 * p1) as i128;
    if d != 0 {
        println!("1");
        let (a, b) = ((da / d) as i128, (db / d) as i128);
        if x1 * a + x2 * b == p1 && y1 * a + y2 * b == p2 {
            return Some((3 * a + b) as i128);
        } else {
            return None;
        }
    } else if da != 0 || db != 0 {
        println!("None");
        return None;
    } else {
        println!("inf");
        let (a, b) = solve_linear(x1, x2, p1);
        return Some(3 * a + b);
    }
}

pub fn solve_linear(x: i128, y: i128, p: i128) -> (i128, i128) {
    let (mut current_a, mut current_b) = (100000, 100000);
    for a in 0..=p / x {
        let b = (p - x * a) / y;
        if p == a * x + b * y && (3 * a + b) < (3 * current_a + current_b) {
            current_a = a;
            current_b = b
        }
    }
    (current_a, current_b)
}

#[aoc(day13, part1)]
pub fn part1(prizes: &Vec<Prize>) -> i128 {
    let mut tokens = 0;
    for p in prizes {
        if let Some(t) = solve(p) {
            dbg!(t);
            tokens += t;
        }
    }
    tokens
}

#[aoc(day13, part2)]
pub fn part2(prizes: &Vec<Prize>) -> i128 {
    let mut tokens = 0;
    for p in prizes {
        let mut new_p = p.clone();
        new_p.coords.0 += 10000000000000;
        new_p.coords.1 += 10000000000000;
        dbg!(&new_p);
        if let Some(t) = solve(&new_p) {
            tokens += t;
        }
    }
    tokens
}
