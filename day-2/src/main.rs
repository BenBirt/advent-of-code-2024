use std::{fs, str::SplitWhitespace};

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-2/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut reports = contents.split("\n");
    let mut safe_reports = 0;
    reports.by_ref().for_each(|report| {
        if is_safe(report.split_whitespace()) {
            safe_reports += 1;
        }
    });

    println!("Part 1: safe_reports={}", safe_reports);
}

fn is_safe(mut levels: SplitWhitespace) -> bool {
    let mut current_level = levels.next().unwrap().parse::<i32>().unwrap();
    let mut increasing_was_set = false;
    let mut increasing = false;
    for next_level_str in levels {
        let next_level = next_level_str.parse::<i32>().unwrap();
        let difference = next_level - current_level;
        if difference == 0 {
            return false;
        }
        if difference > 3 {
            return false;
        }
        if difference < -3 {
            return false;
        }
        if increasing_was_set {
            if (increasing && difference < 0) || (!increasing && difference > 0) {
                return false;
            }
        } else {
            increasing_was_set = true;
            increasing = difference > 0;
        }
        current_level = next_level;
    }
    return true
}
