use std::{fs, ops::Add, sync::LazyLock};

use regex::Regex;

#[derive(Debug)]
struct Button {
    dx: u32,
    dy: u32,
}

static RE: LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"[0-9]+").unwrap());

impl Button {
    fn new(line: &String) -> Button {
        let mut matches = RE.find_iter(line);
        Button {
            dx: matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
            dy: matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
        }
    }

    fn press(&self, times: u64) -> Location {
        Location {
            x: self.dx as u64 * times,
            y: self.dy as u64 * times,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Location {
    x: u64,
    y: u64,
}

impl Location {
    fn new(line: &String) -> Location {
        let mut matches = RE.find_iter(line);
        Location {
            x: matches.nth(0).unwrap().as_str().parse::<u64>().unwrap(),
            y: matches.nth(0).unwrap().as_str().parse::<u64>().unwrap(),
        }
    }

    fn add_offset(&self, x: u64, y: u64) -> Location {
        Location {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Location {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: Location,
}

impl Machine {
    fn cheapest_solution(&self) -> Option<Solution> {
        let a_presses = (self.b.dx as i64 * self.prize.y as i64
            - self.b.dy as i64 * self.prize.x as i64)
            / (self.b.dx as i64 * self.a.dy as i64 - self.b.dy as i64 * self.a.dx as i64);
        let b_presses = (self.prize.x as i64 - self.a.dx as i64 * a_presses) / self.b.dx as i64;

        if a_presses < 0 || b_presses < 0 {
            return Option::None;
        }
        let a_presses = a_presses as u64;
        let b_presses = b_presses as u64;
        let location = self.a.press(a_presses) + self.b.press(b_presses);
        if location == self.prize {
            return Option::Some(Solution { a_presses, b_presses })
        }
        return Option::None;
    }
}

#[derive(Debug)]
struct Solution {
    a_presses: u64,
    b_presses: u64,
}

impl Solution {
    fn cost(&self) -> u64 {
        self.a_presses * 3 + self.b_presses
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-13/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut machines: Vec<Machine> = contents
        .split("\n\n")
        .map(|machine_str| {
            let mut lines = machine_str.split("\n");
            Machine {
                a: Button::new(&lines.nth(0).unwrap().to_string()),
                b: Button::new(&lines.nth(0).unwrap().to_string()),
                prize: Location::new(&lines.nth(0).unwrap().to_string()),
            }
        })
        .collect();

    let total_cost: u64 = machines
        .iter()
        .map(|machine| match machine.cheapest_solution() {
            Option::None => return 0,
            Option::Some(solution) => return solution.cost(),
        })
        .sum();
    println!("Part one: total_cost={}", total_cost);

    machines.iter_mut().for_each(|machine| {
        machine.prize = machine.prize.add_offset(10000000000000, 10000000000000)
    });

    let total_cost: u64 = machines
        .iter()
        .map(|machine| match machine.cheapest_solution() {
            Option::None => return 0,
            Option::Some(solution) => return solution.cost(),
        })
        .sum();
    println!("Part two: total_cost={}", total_cost);
}
