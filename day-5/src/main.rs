use std::{collections::HashMap, fs};

#[derive(Copy, Clone)]

struct Rule {
    first: i32,
    second: i32,
}

impl Rule {
    fn allows_order(&self, first: i32, second: i32) -> bool {
        return self.first == first && self.second == second;
    }
}

impl Rule {}

fn main() {
    let rules_file = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-5/src/rules.txt",
    )
    .expect("Couldn't read input");

    let mut rules = HashMap::new();
    for rule_str in rules_file.split("\n") {
        let mut splitted = rule_str.split("|");
        let first = splitted.nth(0).unwrap().parse::<i32>().unwrap();
        let second = splitted.nth(0).unwrap().parse::<i32>().unwrap();
        let rule = Rule { first, second };
        rules.insert([first, second], rule);
        rules.insert([second, first], rule);
    }

    let updates_file = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-5/src/updates.txt",
    )
    .expect("Couldn't read input");
    let updates: Vec<Vec<i32>> = updates_file
        .split("\n")
        .map(|update_str| {
            return update_str
                .split(",")
                .map(|page_num_str| page_num_str.parse::<i32>().unwrap())
                .collect();
        })
        .collect();

    let mut sum = 0;
    for page_numbers in &updates {
        if update_allowed(&rules, &page_numbers) {
            let middle = page_numbers[page_numbers.len() / 2];
            sum += middle;
        }
    }
    println!("Part 1: sum={}", sum);

    let mut sum = 0;
    for page_numbers in &updates {
        if !update_allowed(&rules, &page_numbers) {
            let new_page_numbers = swap_until_allowed(&rules, &page_numbers);
            let middle = new_page_numbers[new_page_numbers.len() / 2];
            sum += middle;
        }
    }
    println!("Part 2: sum={}", sum);
}

fn update_allowed(rules: &HashMap<[i32; 2], Rule>, page_numbers: &Vec<i32>) -> bool {
    for (i, first_page_number) in page_numbers.iter().enumerate() {
        for second_page_number in &page_numbers[i + 1..] {
            let pair = [*first_page_number, *second_page_number];
            if rules.contains_key(&pair) {
                if !rules[&pair].allows_order(*first_page_number, *second_page_number) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn swap_until_allowed(rules: &HashMap<[i32; 2], Rule>, page_numbers: &Vec<i32>) -> Vec<i32> {
    let mut new_page_numbers = page_numbers.clone();

    for i in 0..new_page_numbers.len() {
        let mut swap_was_required = true;
        while swap_was_required {
            let mut did_swap = false;
            for j in (i + 1)..new_page_numbers.len() {
                let first_page_number = new_page_numbers[i];
                let second_page_number = new_page_numbers[j];
                let pair = [first_page_number, second_page_number];
                if rules.contains_key(&pair) {
                    if !rules[&pair].allows_order(first_page_number, second_page_number) {
                        // swap
                        new_page_numbers[i] = second_page_number;
                        new_page_numbers[j] = first_page_number;
                        // reset
                        did_swap = true;
                        break;
                    }
                }
            }
            if !did_swap {
                swap_was_required = false;
            }
        }
    }
    return new_page_numbers;
}
