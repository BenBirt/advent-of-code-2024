use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Hash, PartialEq, Eq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn difference(&self, other: &Coordinate) -> [i32; 2] {
        return [self.x - other.x, self.y - other.y];
    }

    fn subtract(&self, amount: [i32; 2]) -> Coordinate {
        return Coordinate {
            x: self.x - amount[0],
            y: self.y - amount[1],
        };
    }

    fn add(&self, amount: [i32; 2]) -> Coordinate {
        return Coordinate {
            x: self.x + amount[0],
            y: self.y + amount[1],
        };
    }

    fn is_in_bounds(&self, max_x: usize, max_y: usize) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }
        if self.x as usize > max_x || self.y as usize > max_y {
            return false;
        }
        return true;
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-8/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut antenna_groups = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in contents.split("\n").enumerate() {
        max_y = y;
        for (x, character) in line.chars().enumerate() {
            max_x = x;
            if character != '.' {
                antenna_groups
                    .entry(character)
                    .or_insert(Vec::new())
                    .push(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
            }
        }
    }

    let mut potential_antinodes = HashSet::new();
    for antenna_group in antenna_groups.values() {
        for (i, first_antenna) in antenna_group.iter().enumerate() {
            for second_antenna in antenna_group[i + 1..].iter() {
                let difference = second_antenna.difference(first_antenna);
                potential_antinodes.insert(first_antenna.subtract(difference));
                potential_antinodes.insert(second_antenna.add(difference));
            }
        }
    }
    println!(
        "Part one: count={}",
        potential_antinodes
            .iter()
            .filter(|potential_antinode| potential_antinode.is_in_bounds(max_x, max_y))
            .count()
    );

    let mut antinodes = HashSet::new();
    for antenna_group in antenna_groups.values() {
        for (i, first_antenna) in antenna_group.iter().enumerate() {
            for second_antenna in antenna_group[i + 1..].iter() {
                let difference = second_antenna.difference(first_antenna);
                let mut first_potential_antinode = first_antenna.clone();
                while first_potential_antinode.is_in_bounds(max_x, max_y) {
                    antinodes.insert(first_potential_antinode.clone());
                    first_potential_antinode = first_potential_antinode.subtract(difference);
                }
                let mut second_potential_antinode = second_antenna.clone();
                while second_potential_antinode.is_in_bounds(max_x, max_y) {
                    antinodes.insert(second_potential_antinode.clone());
                    second_potential_antinode = second_potential_antinode.add(difference);
                }
            }
        }
    }
    println!("Part two: count={}", antinodes.len());
}
