use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-2/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut reports = contents.split("\n");
    let mut safe_reports = 0;
    let mut safe_with_skip_reports = 0;
    reports.by_ref().for_each(|report| {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|level_str| level_str.parse::<i32>().unwrap())
            .collect();
        if is_safe(levels.clone()) {
            safe_reports += 1;
            safe_with_skip_reports += 1;
        } else {
            if skip_level(levels.clone())
                .iter()
                .any(|levels_with_skip| is_safe((*levels_with_skip).clone()))
            {
                safe_with_skip_reports += 1;
            }
        }
    });

    println!("Part 1: safe_reports={}", safe_reports);
    println!("Part 2: safe_with_skip_reports={}", safe_with_skip_reports);
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut current_level = levels[0];
    let mut increasing_was_set = false;
    let mut increasing = false;
    for next_level in levels.iter().skip(1) {
        let difference = next_level - current_level;
        if difference == 0 {
            println!("BEN 1");
            return false;
        }
        if difference > 3 {
            println!("BEN 2");
            return false;
        }
        if difference < -3 {
            println!("BEN 3");
            return false;
        }
        if increasing_was_set {
            if (increasing && difference < 0) || (!increasing && difference > 0) {
                println!("BEN 4");
                return false;
            }
        } else {
            increasing_was_set = true;
            increasing = difference > 0;
        }
        current_level = *next_level;
    }
    return true;
}

fn skip_level(levels: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for n in 0..levels.len() {
        result.push(Vec::new());
        for (i, level) in levels.iter().enumerate() {
            if i != n {
                result[n].push(*level);
            }
        }
    }
    return result;
}
