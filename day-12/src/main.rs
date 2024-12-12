use std::{collections::HashSet, fs, ops::Add};

struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn price(&self) -> u32 {
        self.area * self.perimeter
    }
}

impl Add for Region {
    type Output = Region;

    fn add(self, other: Region) -> Region {
        Region {
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
        }
    }
}

struct Map {
    plots: Vec<Vec<String>>,
}

impl Map {
    fn regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for (y, line) in self.plots.iter().enumerate() {
            for (x, region_label) in line.iter().enumerate() {
                if !visited.contains(&(x, y)) {
                    regions.push(self.explore_region(
                        &mut visited,
                        (x as i32, y as i32),
                        region_label,
                    ));
                }
            }
        }
        return regions;
    }

    fn explore_region(
        &self,
        visited: &mut HashSet<(usize, usize)>,
        (x, y): (i32, i32),
        region_label: &String,
    ) -> Region {
        // check if we fell out of the region
        if x < 0 || y < 0 {
            return Region {
                area: 0,
                perimeter: 1,
            };
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.plots[0].len() || y >= self.plots.len() {
            return Region {
                area: 0,
                perimeter: 1,
            };
        }
        if self.plots[y][x] != *region_label {
            return Region {
                area: 0,
                perimeter: 1,
            };
        }

        // check if we already visited the location (which must be in this region, by definition)
        if visited.contains(&(x, y)) {
            return Region {
                area: 0,
                perimeter: 0,
            };
        }

        // we must be in the same region, and we must be exploring something new.
        visited.insert((x, y));
        let mut this_region = Region {
            area: 1,
            perimeter: 0,
        };
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            this_region = this_region
                + self.explore_region(visited, (x as i32 + dx, y as i32 + dy), region_label);
        }
        return this_region;
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-12/src/input.txt",
    )
    .expect("Couldn't read input");

    let plots = contents
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|character| character.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let total_cost: u32 = Map{plots}.regions().iter().map(|region| region.price()).sum();
    println!("Part one: total_cost={}", total_cost);
}
