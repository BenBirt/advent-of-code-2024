use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-10/src/input.txt",
    )
    .expect("Couldn't read input");

    let map: Vec<Vec<u32>> = contents
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut reachable = vec![vec![Option::None; map[0].len()]; map.len()];
    let mut ratings = vec![vec![-1; map[0].len()]; map.len()];

    let mut total_trailhead_score = 0;
    let mut total_rating = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, digit) in line.iter().enumerate() {
            if *digit == 0 {
                let (reachable_from_here, this_rating) =
                    compute_reachability(&map, &mut reachable, &mut ratings, y as i32, x as i32, 0);
                total_trailhead_score += reachable_from_here.len();
                total_rating += this_rating;
            }
        }
    }
    println!("Part one: total_trailhead_score={}", total_trailhead_score);
    println!("Part two: total_rating={}", total_rating);
}

fn compute_reachability(
    map: &Vec<Vec<u32>>,
    reachable: &mut Vec<Vec<Option<HashSet<[i32; 2]>>>>,
    ratings: &mut Vec<Vec<i32>>,
    y: i32,
    x: i32,
    required_height: u32,
) -> (HashSet<[i32; 2]>, i32) {
    if y < 0 || x < 0 {
        return (HashSet::new(), 0);
    }
    let y = y as usize;
    let x = x as usize;
    if y >= map.len() || x >= map[0].len() {
        return (HashSet::new(), 0);
    }
    if map[y][x] != required_height {
        return (HashSet::new(), 0);
    }
    if reachable[y][x].is_none() {
        let mut new_reachable = HashSet::new();
        let mut new_rating = 0;
        if map[y][x] == 9 {
            new_reachable.insert([y as i32, x as i32]);
            new_rating = 1;
        } else {
            let deltas: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
            for [dy, dx] in deltas {
                let (reachable_from_there, that_rating) = compute_reachability(
                    map,
                    reachable,
                    ratings,
                    y as i32 + dy,
                    x as i32 + dx,
                    required_height + 1,
                );
                for newly_reachable in reachable_from_there {
                    new_reachable.insert(newly_reachable);
                }
                new_rating += that_rating;
            }
        }
        reachable[y][x] = Option::Some(new_reachable);
        ratings[y][x] = new_rating;
    }
    return (reachable[y][x].clone().unwrap(), ratings[y][x]);
}
