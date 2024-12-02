#![feature(str_split_whitespace_remainder)]

use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-1/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut group_1: Vec<i32> = Vec::new();
    let mut group_2: Vec<i32> = Vec::new();

    let mut pairs = contents.split("\n");
    pairs.by_ref().for_each(|line| {
        let mut location_ids = line.split_whitespace();
        group_1.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        group_2.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        assert_eq!(location_ids.remainder(), None);
    });

    group_1.sort();
    group_2.sort();

    let mut total_distance = 0;
    let it = group_1.iter().zip(group_2.iter());
    for (_, (location_id_1, location_id_2)) in it.enumerate() {
        let distance = (location_id_1 - location_id_2).abs();
        total_distance += distance;
    }
    println!("total_distance={}", total_distance);
}
