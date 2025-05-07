use std::{fmt, iter::repeat};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u8> {
    let mut v = vec![];
    for ch in input.chars() {
        if ch.is_numeric() {
            v.push(ch.to_digit(10).unwrap() as u8);
        }
    }
    v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    Occupied(u64),
    Empty,
}

impl Block {
    fn is_occupied(&self) -> bool {
        match self {
            Self::Occupied(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Occupied(val) => write!(f, "{}", val),
            _ => write!(f, "."),
        }
    }
}
pub fn to_blocks(disk_map: &Vec<u8>) -> Vec<Block> {
    let mut id = 0u64;
    let mut blocks = vec![];
    let chunks_iter = disk_map.chunks_exact(2);
    for pair in chunks_iter {
        if let [occupied_size, free_size] = pair {
            let occupied_block: Vec<Block> = repeat(id)
                .take(*occupied_size as usize)
                .map(|x| Block::Occupied(x))
                .collect();
            let free_block: Vec<Block> = repeat('.')
                .take(*free_size as usize)
                .map(|_| Block::Empty)
                .collect();
            blocks.extend(occupied_block);
            blocks.extend(free_block);
            id += 1
        } else {
            panic!()
        }
    }
    let remainder = disk_map.chunks_exact(2).remainder();
    if !remainder.is_empty() {
        let occupied_size = remainder[0];
        let occupied_block: Vec<Block> = repeat(id)
            .take(occupied_size as usize)
            .map(|x| Block::Occupied(x))
            .collect();
        blocks.extend(occupied_block);
    }
    blocks
}

pub fn move_block(blocks: &mut Vec<Block>) {
    let mut right = blocks.len() - 1;
    let mut left = blocks.iter().position(|&b| !b.is_occupied()).unwrap();
    while left < right {
        match (blocks[left], blocks[right]) {
            (Block::Empty, Block::Occupied(_)) => {
                blocks.swap(left, right);
                left += 1;
                right -= 1;
            }

            (Block::Empty, Block::Empty) => right -= 1,
            (Block::Occupied(_), Block::Empty) => {
                left += 1;
                right -= 1;
            }
            (Block::Occupied(_), Block::Occupied(_)) => left += 1,
        }
    }
}

pub fn checksum(blocks: &Vec<Block>) -> u128 {
    let mut checksum = 0;
    for (i, b) in blocks.iter().enumerate() {
        match b {
            Block::Occupied(id) => checksum += i as u128 * *id as u128,
            Block::Empty => {}
        }
    }
    checksum
}

pub fn print_blocks(blocks: &Vec<Block>) {
    blocks.iter().for_each(|x| print!("{}", x));
    println!()
}
#[aoc(day9, part1)]
pub fn part1(disk_map: &Vec<u8>) -> u128 {
    let mut blocks = to_blocks(disk_map);
    print_blocks(&blocks);
    dbg!(find_occupied_segment(&blocks, max_id(&blocks)));
    move_block(&mut blocks);
    print_blocks(&blocks);
    checksum(&blocks)
}

pub fn max_id(blocks: &Vec<Block>) -> u64 {
    blocks
        .iter()
        .filter_map(|b| match b {
            Block::Occupied(id) => Some(*id),
            Block::Empty => None,
        })
        .max()
        .unwrap_or(0)
}

pub fn find_occupied_segment(blocks: &Vec<Block>, id: u64) -> Option<(usize, usize)> {
    let mut start: Option<usize> = None;
    let mut length = 0usize;
    for (i, block) in blocks.iter().enumerate() {
        match block {
            Block::Occupied(current_id) if *current_id == id => {
                if start.is_none() {
                    start = Some(i);
                }
                length += 1;
            }
            _ => {
                if start.is_some() && length > 0 {
                    break;
                }
            }
        }
    }
    start.map(|s| (s, length))
}

#[aoc(day9, part2)]
pub fn part2(disk_map: &Vec<u8>) -> u128 {
    let mut blocks = to_blocks(disk_map);
    let max_id = max_id(&blocks);
    for current_id in (0..=max_id).rev() {
        let segment = find_occupied_segment(&blocks, current_id);

        if let Some((file_start, file_len)) = segment {
            if file_len > 0 {
                // --- Find the target empty slot ---
                let mut target_start_opt: Option<usize> = None;
                let mut current_empty_start: Option<usize> = None;
                let mut current_empty_len = 0;

                // Search *only to the left* of the file (indices 0 to file_start - 1)
                for i in 0..file_start {
                    if blocks[i] == Block::Empty {
                        if current_empty_start.is_none() {
                            current_empty_start = Some(i); // Mark start of potential slot
                        }
                        current_empty_len += 1;
                    } else {
                        // Just finished a sequence of empty blocks (or hit an occupied one)
                        if let Some(start) = current_empty_start {
                            if current_empty_len >= file_len {
                                // Found a suitable slot! It's the leftmost one so far.
                                target_start_opt = Some(start);
                                break; // Stop searching immediately
                            }
                        }
                        // Reset for the next potential empty sequence
                        current_empty_start = None;
                        current_empty_len = 0;
                    }
                }

                // Check if the search ended while in a potential empty block sequence
                if target_start_opt.is_none() {
                    // Only if we haven't already found a slot
                    if let Some(start) = current_empty_start {
                        if current_empty_len >= file_len {
                            target_start_opt = Some(start);
                        }
                    }
                }

                // --- Perform the move if a target was found ---
                if let Some(target_start) = target_start_opt {
                    // Move the file:
                    // 1. Copy the file blocks (Block::Occupied(current_id)) to a temp buffer or note the ID/length.
                    // 2. Overwrite the original file location (file_start to file_start + file_len - 1) with Block::Empty.
                    // 3. Overwrite the target location (target_start to target_start + file_len - 1) with Block::Occupied(current_id).

                    // Example implementation (using simple loops):
                    let file_block_type = Block::Occupied(current_id);
                    let empty_block_type = Block::Empty;

                    // Clear original spot
                    for i in 0..file_len {
                        blocks[file_start + i] = empty_block_type;
                    }
                    // Fill target spot
                    for i in 0..file_len {
                        blocks[target_start + i] = file_block_type;
                    }

                    // Optional: Print state after move for debugging
                    // println!("Moved file {}:", current_id);
                    // print_blocks(&blocks);
                }
                // else: No suitable slot found, file `current_id` does not move.
            }
        }
    }
    checksum(&blocks)
}
