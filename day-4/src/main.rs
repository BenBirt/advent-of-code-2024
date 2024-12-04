use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-4/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut word_search = Vec::new();
    for line in contents.split("\n") {
        let mut word_search_line = Vec::new();
        for c in line.chars() {
            word_search_line.push(c);
        }
        word_search.push(word_search_line);
    }

    let directions: [i32; 3] = [-1, 0, 1];
    let mut count = 0;
    for y in 0..word_search.len() {
        for x in 0..word_search[0].len() {
            for dy in directions {
                for dx in directions {
                    if search(&word_search, "XMAS", y, dy, x, dx) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: count={}", count);
}

fn search(
    word_search: &Vec<Vec<char>>,
    search_str: &str,
    y: usize,
    dy: i32,
    x: usize,
    dx: i32,
) -> bool {
    if word_search[y][x] != search_str.chars().nth(0).unwrap() {
        return false;
    }
    if search_str.len() == 1 {
        // We found our last character.
        return true;
    }

    let new_y = y as i32 + dy;
    if new_y < 0 {
        return false;
    }
    let new_y = new_y as usize;
    if new_y >= word_search.len() {
        return false;
    }

    let new_x = x as i32 + dx;
    if new_x < 0 {
        return false;
    }
    let new_x = new_x as usize;
    if new_x >= word_search[0].len() {
        return false;
    }

    return search(word_search, &search_str[1..], new_y, dy, new_x, dx);
}
