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
        .map(|re_match| do_mul(re_match.as_str()))
        .sum();
    println!("Part one: sum={}", sum);

    let re2 = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let sum2: i32 = re2
        .find_iter(&contents)
        .map(|re2_match| {
            match re2_match.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        return do_mul(re2_match.as_str());
                    }
                }
            }
            return 0;
        })
        .sum();
    println!("Part two: sum={}", sum2);
}

fn do_mul(input: &str) -> i32 {
    let split: Vec<&str> = input.split(",").collect();
    let first_num = &split[0][4..].parse::<i32>().unwrap();
    let second_num = &split[1][..&split[1].len() - 1].parse::<i32>().unwrap();
    return first_num * second_num;
}
