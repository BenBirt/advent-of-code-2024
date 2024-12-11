use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-11/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut stones: Vec<String> = contents
        .split_whitespace()
        .map(|stone| stone.to_string())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in stones.iter() {
            if *stone == "0" {
                new_stones.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                let midpoint = stone.len() / 2;
                let left = &stone[..midpoint];
                let mut right = &stone[midpoint..];
                while right.starts_with("0") && right.len() > 1 {
                    right = &right[1..];
                }
                new_stones.push(left.to_string());
                new_stones.push(right.to_string());
            } else {
                let parsed = stone.parse::<u64>();
                if parsed.is_err() {
                    panic!("error parsing {}", stone);
                }
                let value = stone.parse::<u64>().unwrap() * 2024;
                new_stones.push(value.to_string());
            }
        }
        stones = new_stones;
    }
    println!("Part one: count={}", stones.len());
}
