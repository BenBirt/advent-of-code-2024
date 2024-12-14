use std::{fs, sync::LazyLock};

use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: (u32, u32),
    velocity: (i32, i32),
}

static RE: LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"-?[0-9]+").unwrap());

impl Robot {
    fn new(line: &str) -> Robot {
        let mut matches = RE.find_iter(line);
        let position = (
            matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
            matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
        );
        let velocity = (
            parse_possibly_negative_number(matches.nth(0).unwrap().as_str()),
            parse_possibly_negative_number(matches.nth(0).unwrap().as_str()),
        );

        Robot { position, velocity }
    }

    fn step(&mut self, x_len: u32, y_len: u32) {
        let next_x = (self.position.0 as i32 + self.velocity.0).rem_euclid(x_len as i32) as u32;
        let next_y = (self.position.1 as i32 + self.velocity.1).rem_euclid(y_len as i32) as u32;
        self.position = (next_x, next_y);
    }
}

fn parse_possibly_negative_number(mut number_str: &str) -> i32 {
    let mut negative = false;
    if number_str.starts_with("-") {
        number_str = &number_str[1..];
        negative = true;
    }
    (if negative { -1 } else { 1 }) * number_str.parse::<i32>().unwrap()
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-14/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut robots: Vec<Robot> = contents.split("\n").map(|line| Robot::new(line)).collect();

    let x_len = 101;
    let y_len = 103;

    robots.iter_mut().for_each(|robot| {
        for _ in 0..100 {
            robot.step(x_len, y_len);
        }
    });

    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;
    for robot in robots {
        if robot.position.0 < x_len/2 {
            if robot.position.1 < y_len/2 {
                quadrant_1 += 1;
            } else if robot.position.1 > y_len/2 {
                quadrant_2 += 1;
            }
        } else if robot.position.0 > x_len/2 {
            if robot.position.1 < y_len/2 {
                quadrant_3 += 1;
            } else if robot.position.1 > y_len/2 {
                quadrant_4 += 1;
            }
        }
    }
    println!("Part one: total safety factor={}", quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4);
}
