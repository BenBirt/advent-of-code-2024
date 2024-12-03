use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-3/src/input.txt",
    )
    .expect("Couldn't read input");
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let sum: i32 = re
        .find_iter(&contents)
        .map(|re_match| {
            let split: Vec<&str> = re_match.as_str().split(",").collect();
            let first_num = &split[0][4..].parse::<i32>().unwrap();
            let second_num = &split[1][..&split[1].len() - 1].parse::<i32>().unwrap();
            return first_num * second_num;
        })
        .sum();
    println!("Part one: sum={}", sum);
}
