use std::{collections::HashMap, fs};

type StoneDepth = (String, u64);

struct StoneResolver {
    memoized: HashMap<StoneDepth, u64>,
}

impl StoneResolver {
    fn new() -> StoneResolver {
        return StoneResolver {
            memoized: HashMap::new(),
        };
    }

    fn resolve(&mut self, label: String, depth: u64) -> u64 {
        let key = (label.clone(), depth);
        if self.memoized.contains_key(&key) {
            return self.memoized[&key];
        }

        if depth == 0 {
            return 1;
        }

        if label == "0" {
            let value = self.resolve("1".to_string(), depth - 1);
            self.memoized.insert(key, value);
            return value;
        }

        if label.len() % 2 == 0 {
            let midpoint = label.len() / 2;
            let left = &label[..midpoint];
            let mut right = &label[midpoint..];
            while right.starts_with("0") && right.len() > 1 {
                right = &right[1..];
            }
            let value = self.resolve(left.to_string(), depth - 1)
                + self.resolve(right.to_string(), depth - 1);
            self.memoized.insert(key, value);
            return value;
        }

        let new_label = (label.parse::<u64>().unwrap() * 2024).to_string();
        let value = self.resolve(new_label, depth - 1);
        self.memoized.insert(key, value);
        return value;
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-11/src/input.txt",
    )
    .expect("Couldn't read input");

    let stones: Vec<String> = contents
        .split_whitespace()
        .map(|stone| stone.to_string())
        .collect();

    let mut resolver = StoneResolver::new();
    let mut stone_count = 0;
    for stone in stones.clone() {
        stone_count += resolver.resolve(stone, 25);
    }
    println!("Part one: count={}", stone_count);
    let mut stone_count = 0;
    for stone in stones {
        stone_count += resolver.resolve(stone, 75);
    }
    println!("Part two: count={}", stone_count);
}
