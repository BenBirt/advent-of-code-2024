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

    fn press(&self, times: u32) -> Location {
        Location {
            x: self.dx * times,
            y: self.dy * times,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Location {
    x: u32,
    y: u32,
}

impl Location {
    fn new(line: &String) -> Location {
        let mut matches = RE.find_iter(line);
        Location {
            x: matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
            y: matches.nth(0).unwrap().as_str().parse::<u32>().unwrap(),
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
        let mut cheapest_solution: Option<Solution> = Option::None;
        for a_presses in 0..101 {
            let location_from_pressing_a = self.a.press(a_presses);
            if location_from_pressing_a.x > self.prize.x
                || location_from_pressing_a.y > self.prize.y
            {
                break;
            }
            for b_presses in 0..101 {
                let location = location_from_pressing_a + self.b.press(b_presses);
                if location.x > self.prize.x || location.y > self.prize.y {
                    break;
                }
                if location == self.prize {
                    let new_solution = Solution {
                        a_presses,
                        b_presses,
                    };
                    match cheapest_solution {
                        Option::None => cheapest_solution = Option::Some(new_solution),
                        Option::Some(ref old_solution) => {
                            if new_solution.cost() < old_solution.cost() {
                                cheapest_solution = Option::Some(new_solution)
                            }
                        }
                    }
                }
            }
        }
        return cheapest_solution;
    }
}

#[derive(Debug)]
struct Solution {
    a_presses: u32,
    b_presses: u32,
}

impl Solution {
    fn cost(&self) -> u32 {
        self.a_presses * 3 + self.b_presses
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-13/src/input.txt",
    )
    .expect("Couldn't read input");

    let machines: Vec<Machine> = contents
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

    let total_cost: u32 = machines.iter().map(|machine| {
        match machine.cheapest_solution() {
            Option::None => return 0,
            Option::Some(solution) => return solution.cost(),
        }
    }).sum();
    println!("Part one: total_cost={}", total_cost);
}
