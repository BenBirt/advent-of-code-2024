use std::{collections::HashSet, fs, ops::Add};

struct Region {
    label: String,
    root: (usize, usize),
    area: u32,
    perimeter: u32,
    perimeter_locations: HashSet<(usize, usize)>,
}

impl Region {
    fn price(&self) -> u32 {
        self.area * self.perimeter
    }
}

impl Add for Region {
    type Output = Region;

    fn add(self, other: Region) -> Region {
        let mut new_perimeter_locations = self.perimeter_locations;
        for other_perimeter_location in other.perimeter_locations {
            new_perimeter_locations.insert(other_perimeter_location);
        }
        Region {
            label: self.label,
            root: self.root,
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
            perimeter_locations: new_perimeter_locations,
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
                    regions.push(
                        self.explore_region(&mut visited, (x as i32, y as i32), region_label)
                            .unwrap(),
                    );
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
    ) -> Option<Region> {
        // check if we fell out of the region
        if !self.is_in_region((x, y), region_label) {
            return Option::None;
        }
        let x = x as usize;
        let y = y as usize;

        // check if we already visited the location (which must be in this region, by definition)
        if visited.contains(&(x, y)) {
            return Option::Some(Region {
                label: (*region_label).clone(),
                root: (0, 0),
                area: 0,
                perimeter: 0,
                perimeter_locations: HashSet::new(),
            });
        }

        // we must be in the same region, and we must be exploring something new.
        visited.insert((x, y));
        let mut this_region = Region {
            label: (*region_label).clone(),
            root: (x, y),
            area: 1,
            perimeter: 0,
            perimeter_locations: HashSet::new(),
        };
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            match self.explore_region(visited, (x as i32 + dx, y as i32 + dy), region_label) {
                Option::Some(other_region) => {
                    this_region = this_region + other_region;
                }
                Option::None => {
                    this_region.perimeter += 1;
                    this_region.perimeter_locations.insert((x, y));
                }
            }
        }
        return Option::Some(this_region);
    }

    fn is_in_region(&self, (x, y): (i32, i32), region_label: &String) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.plots[0].len() || y >= self.plots.len() {
            return false;
        }
        if self.plots[y][x] != *region_label {
            return false;
        }
        return true;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(&self, (x, y): (usize, usize)) -> (i32, i32) {
        match self {
            Direction::Up => (x as i32, y as i32 - 1),
            Direction::Right => (x as i32 + 1, y as i32),
            Direction::Down => (x as i32, y as i32 + 1),
            Direction::Left => (x as i32 - 1, y as i32),
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// static mut F_COUNTER: i32 = 0;

fn walk_perimeter(map: &Map, region: &Region) -> u32 {
    let mut should_see_all_points_and_directions_on_perimeter = HashSet::new();
    for point in region.perimeter_locations.iter() {
        for direction in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if !map.is_in_region(direction.step(*point), &region.label) {
                should_see_all_points_and_directions_on_perimeter
                    .insert((*point, direction.turn_right()));
            }
        }
    }

    let mut sides = 0;
    while !should_see_all_points_and_directions_on_perimeter.is_empty() {
        let (starting_point, starting_direction) =
            *should_see_all_points_and_directions_on_perimeter
                .iter()
                .nth(0)
                .unwrap();
        should_see_all_points_and_directions_on_perimeter
            .remove(&(starting_point, starting_direction));

        let mut current_point = starting_point;
        let mut current_direction = starting_direction.clone();
        let mut direction_changes = 0;
        loop {
            let left_point = current_direction.turn_left().step(current_point);
            let straight_on_point = current_direction.step(current_point);
            if map.is_in_region(left_point, &region.label) {
                current_point = (left_point.0 as usize, left_point.1 as usize);
                current_direction = current_direction.turn_left();
                direction_changes += 1;
            } else if map.is_in_region(straight_on_point, &region.label) {
                current_point = (straight_on_point.0 as usize, straight_on_point.1 as usize);
            } else {
                current_direction = current_direction.turn_right();
                direction_changes += 1;
            }

            should_see_all_points_and_directions_on_perimeter
                .remove(&(current_point, current_direction));

            if current_point == starting_point && current_direction == starting_direction {
                break;
            }
        }
        sides += direction_changes;
    }
    return sides;
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

    let map = Map { plots };
    let regions = map.regions();

    let total_cost: u32 = regions.iter().map(|region| region.price()).sum();
    println!("Part one: total_cost={}", total_cost);

    let total_cost: u32 = regions
        .iter()
        .map(|region| {
            let sides = walk_perimeter(&map, region);
            return sides * region.area;
        })
        .sum();
    println!("Part two: total_cost={}", total_cost);
}
