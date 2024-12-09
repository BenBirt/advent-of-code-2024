use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Block {
    Filled(u32),
    Empty,
}

struct FileData {
    file_number: u32,
    length: u32,
    location: usize,
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-9/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut blocks = Vec::new();
    let mut file_data_map = HashMap::new();
    for (i, character) in contents.chars().enumerate() {
        let file_index = i as u32 / 2;
        let to_push = if i % 2 == 0 {
            Block::Filled(file_index)
        } else {
            Block::Empty
        };
        let num_blocks = character.to_digit(10).unwrap();
        if to_push != Block::Empty {
            file_data_map.insert(
                file_index,
                FileData {
                    file_number: file_index,
                    length: num_blocks,
                    location: blocks.len(),
                },
            );
        }
        for _ in 0..num_blocks {
            blocks.push(to_push.clone());
        }
    }

    let mut blocks_part_one_clone = blocks.clone();
    let mut left_index = 0;
    let mut right_index = blocks_part_one_clone.len() - 1;
    loop {
        if left_index == right_index || left_index >= blocks_part_one_clone.len() {
            break;
        }
        if blocks_part_one_clone[left_index] != Block::Empty {
            left_index += 1;
            continue;
        }
        if blocks_part_one_clone[right_index] == Block::Empty {
            right_index -= 1;
            continue;
        }
        blocks_part_one_clone.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;
    }

    let checksum: u64 = blocks_part_one_clone
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::Empty => return 0,
            Block::Filled(file_number) => {
                return index as u64 * (*file_number as u64);
            }
        })
        .sum();

    println!("Part one: checksum={}", checksum);

    let mut blocks_part_two_clone = blocks.clone();
    for file_number in (1..file_data_map.len() as u32).rev() {
        let file_data = &file_data_map[&file_number];

        let free_space = find_free_space(
            &blocks_part_two_clone[0..file_data.location].to_vec(),
            file_data.length,
        );
        if free_space.is_none() {
            continue;
        }
        let free_space_index = free_space.unwrap();
        for i in 0..file_data.length {
            blocks_part_two_clone.swap(
                file_data.location + i as usize,
                free_space_index + i as usize,
            );
        }
        file_data_map.insert(
            file_data.file_number,
            FileData {
                file_number: file_data.file_number,
                length: file_data.length,
                location: free_space_index,
            },
        );
    }

    let checksum: u64 = blocks_part_two_clone
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::Empty => return 0,
            Block::Filled(file_number) => {
                return index as u64 * (*file_number as u64);
            }
        })
        .sum();

    println!("Part two: checksum={}", checksum);
}

fn find_free_space(blocks: &Vec<Block>, size: u32) -> Option<usize> {
    let mut found_index = 0;
    let mut found_size = 0;
    for (index, block) in blocks.iter().enumerate() {
        match block {
            Block::Empty => {
                if found_size == 0 {
                    found_index = index;
                }
                found_size += 1;
            }
            Block::Filled(_) => {
                found_size = 0;
            }
        }
        if found_size >= size {
            return Option::Some(found_index);
        }
    }
    return Option::None;
}
