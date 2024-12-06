use std::{collections::{HashMap, HashSet}, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_out_of_bounds(&self, max_x: usize, max_y: usize) -> bool {
        if self.x < 0 || self.y < 0 {
            return true;
        }
        if self.x as usize > max_x || self.y as usize > max_y {
            return true;
        }
        return false;
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(&self, position: &Position) -> Position {
        match self {
            Direction::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
        }
    }
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-6/src/input.txt",
    )
    .expect("Couldn't read input");

    let mut map = Vec::new();
    let mut start: Position = Position { x: 0, y: 0 };
    for (y, line_str) in contents.split("\n").into_iter().enumerate() {
        let mut line = Vec::new();
        for (x, char) in line_str.chars().into_iter().enumerate() {
            line.push(char);
            if char == '^' {
                start = Position {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
        map.push(line);
    }

    println!("Part one: count={}", follow_path(&map, start.clone()));
}

fn follow_path(map: &Vec<Vec<char>>, start: Position) -> i32 {
    let mut visited = HashMap::new();
    let mut current_pos = start;
    let mut current_direction = Direction::Up;
    loop {
        // mark
        let visited_directions_for_current_pos = visited.entry(current_pos).or_insert(HashSet::new());
        if visited_directions_for_current_pos.contains(&current_direction) {
            // we must be in a loop. eject!
            return -1;
        }
        visited_directions_for_current_pos.insert(current_direction.clone());

        // take next step
        let next_pos = current_direction.step(&current_pos);

        // exit condition
        if next_pos.is_out_of_bounds(map[0].len() - 1, map.len() - 1) {
            break;
        }

        match map[next_pos.y as usize][next_pos.x as usize] {
            '#' => {
                current_direction = current_direction.turn();
            }
            'X' => {
                current_pos = next_pos;
            }
            '.' => {
                current_pos = next_pos;
            }
            _ => panic!(
                "encountered {} at (x, y)=({}, {})",
                map[next_pos.y as usize][next_pos.x as usize], next_pos.x, next_pos.y
            ),
        }
    }
    return visited.len() as i32;
}
