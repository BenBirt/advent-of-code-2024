#![feature(str_split_whitespace_remainder)]

use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-1/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut group_1 = Vec::new();
    let mut group_2 = Vec::new();

    let mut pairs = contents.split("\n");
    pairs.by_ref().for_each(|line| {
        let mut location_ids = line.split_whitespace();
        group_1.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        group_2.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        assert_eq!(location_ids.remainder(), None);
    });

    // part_one(group_1, group_2);
    part_two(group_1, group_2);
}

fn part_one(mut group_1: Vec<i32>, mut group_2: Vec<i32>) {
    group_1.sort();
    group_2.sort();

    let mut total_distance = 0;
    let it = group_1.iter().zip(group_2.iter());
    for (_, (location_id_1, location_id_2)) in it.enumerate() {
        let distance = (location_id_1 - location_id_2).abs();
        total_distance += distance;
    }
    println!("Part 1: total_distance={}", total_distance);
}

fn part_two(group_1: Vec<i32>, group_2: Vec<i32>) {
    let mut group_2_counts: HashMap<i32, i32> = HashMap::new();
    for location_id in group_2.iter() {
        let count = group_2_counts.entry(*location_id).or_insert(0);
        *count += 1;
    }

    let mut total_similarity = 0;
    for location_id in group_1.iter() {
        if group_2_counts.contains_key(location_id) {
            let similarity = location_id * group_2_counts[location_id];
            total_similarity += similarity;
        }
    }
    println!("Part 2: total_similarity={}", total_similarity);
}
