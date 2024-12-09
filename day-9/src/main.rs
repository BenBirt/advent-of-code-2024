use std::fs;

#[derive(PartialEq, Eq, Clone)]
enum Block {
    Filled(u32),
    Empty,
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-9/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut blocks = Vec::new();
    let mut file_counter = 0;
    for (i, character) in contents.chars().enumerate() {
        let to_push = if i % 2 == 0 {
            Block::Filled(file_counter)
        } else {
            Block::Empty
        };
        let num_blocks = character.to_digit(10).unwrap();
        for _ in 0..num_blocks {
            blocks.push(to_push.clone());
        }
        if i % 2 == 1 {
            file_counter += 1;
        }
    }

    let mut left_index = 0;
    let mut right_index = blocks.len() - 1;
    loop {
        if left_index == right_index || left_index >= blocks.len() {
            break;
        }
        if blocks[left_index] != Block::Empty {
            left_index += 1;
            continue;
        }
        if blocks[right_index] == Block::Empty {
            right_index -= 1;
            continue;
        }
        blocks.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;
    }

    let checksum: u64 = blocks.iter().enumerate()
        .map(|(index, block)| {
            match block {
                Block::Empty => { return 0 },
                Block::Filled(file_number) => {
                    return index as u64 * (*file_number as u64);
                }
            }
        })
        .sum();

    println!("Part one: checksum={}", checksum);
}
