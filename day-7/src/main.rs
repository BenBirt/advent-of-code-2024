use std::{collections::HashSet, fs};

struct Equation {
    numbers: Vec<i64>,
    result: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn compute(numbers: &Vec<i64>, operators: &Vec<Operator>) -> i64 {
    let mut numbers_iter = numbers.iter();
    let mut operators_iter = operators.iter();
    let mut current_result = *(numbers_iter.next().unwrap());
    let mut next = numbers_iter.next();
    while next != Option::None {
        match operators_iter.next().unwrap() {
            Operator::Add => current_result += next.unwrap(),
            Operator::Multiply => current_result *= next.unwrap(),
            Operator::Concatenate => {
                current_result = format!("{}{}", current_result, next.unwrap())
                    .parse::<i64>()
                    .unwrap()
            }
        }
        next = numbers_iter.next();
    }
    return current_result;
}

fn can_satisfy(
    result: i64,
    possible_operators: &HashSet<Operator>,
    numbers: &Vec<i64>,
    operators: &mut Vec<Operator>,
) -> bool {
    if operators.len() == numbers.len() - 1 {
        return compute(numbers, operators) == result;
    }

    for possible_operator in possible_operators {
        operators.push(*possible_operator);
        if can_satisfy(result, possible_operators, numbers, operators) {
            return true;
        }
        operators.pop();
    }
    return false;
}

impl Equation {
    fn find_operators_to_satisfy(
        &self,
        possible_operators: &HashSet<Operator>,
    ) -> Option<Vec<Operator>> {
        let mut operators = Vec::new();
        if can_satisfy(
            self.result,
            possible_operators,
            &self.numbers,
            &mut operators,
        ) {
            return Option::Some(operators);
        }
        return Option::None;
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/usr/local/google/home/benbirt/github/advent-of-code-2024/day-7/src/input.txt",
    )
    .expect("Couldn't read input");

    let equations: Vec<Equation> = contents
        .split("\n")
        .map(|line| {
            let mut splitted = line.split(": ");
            let result_str = splitted.nth(0).unwrap().to_owned();
            let result = result_str
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("failed to parse: {}", result_str));
            let numbers = splitted
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();
            return Equation { numbers, result };
        })
        .collect();

    let mut sum = 0;
    for equation in &equations {
        if equation
            .find_operators_to_satisfy(&HashSet::from([Operator::Add, Operator::Multiply]))
            .is_some()
        {
            sum += equation.result;
        }
    }
    println!("Part one: sum={}", sum);

    let mut sum = 0;
    for equation in &equations {
        if equation
            .find_operators_to_satisfy(&HashSet::from([
                Operator::Add,
                Operator::Multiply,
                Operator::Concatenate,
            ]))
            .is_some()
        {
            sum += equation.result;
        }
    }
    println!("Part two: sum={}", sum);
}
