use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::{HashMap, LinkedList};

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn build_matrix(cargo_str: &str) -> Vec<Vec<&str>> {
    cargo_str
        .lines()
        .map(|line| line.split("").filter(|s| s != &"").collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn invert_cargo(cargo_matrix: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    transpose(cargo_matrix)
        .into_iter()
        .map(|row| row.into_iter().rev().collect::<Vec<&str>>())
        .collect::<Vec<Vec<_>>>()
}

fn build_stacks(cargo_matrix: Vec<Vec<&str>>) -> HashMap<i32, LinkedList<&str>> {
    let mut map: HashMap<i32, LinkedList<&str>> = HashMap::new();
    cargo_matrix.into_iter().for_each(|row| {
        let first_symbol = row.first().and_then(|s| s.parse::<i32>().ok());
        if let Some(stack_num) = first_symbol {
            let list =
                LinkedList::from_iter(row.into_iter().skip(1).filter(|s| !s.trim().is_empty()));
            map.insert(stack_num, list);
        }
    });
    map
}

fn parse_capture(cap: Option<Match>) -> Option<i32> {
    cap.and_then(|x| x.as_str().parse::<i32>().ok())
}

struct Command {
    amount: i32,
    stack_from_num: i32,
    stack_to_num: i32,
}

impl Command {
    fn from(command: &str) -> Command {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"move (?P<amount>\d+) from (?P<stack_from_num>\d+) to (?P<stack_to_num>\d+)"
            )
            .unwrap();
        }

        RE.captures(command)
            .map(|cap| Command {
                amount: parse_capture(cap.name("amount")).unwrap(),
                stack_from_num: parse_capture(cap.name("stack_from_num")).unwrap(),
                stack_to_num: parse_capture(cap.name("stack_to_num")).unwrap(),
            })
            .unwrap()
    }
}

fn move_stacks<'a, 'b>(
    mut stacks: HashMap<i32, LinkedList<&'a str>>,
    command: &'b Command,
) -> HashMap<i32, LinkedList<&'a str>> {
    let from_stack = stacks.get_mut(&command.stack_from_num).unwrap();
    let items: Vec<_> = (0..command.amount)
        .map(|_| from_stack.pop_back().unwrap())
        .into_iter()
        .collect();
    let to_stack = stacks.get_mut(&command.stack_to_num).unwrap();
    items.iter().for_each(|item| to_stack.push_back(item));
    stacks
}

fn get_result(stacks: HashMap<i32, LinkedList<&str>>) -> String {
    let mut keys: Vec<_> = stacks.keys().collect();
    keys.sort();
    keys.iter()
        .map(|key| *(stacks.get(key).and_then(|stack| stack.back()).unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

pub fn part_one(input: &str) -> Option<String> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let cargo = parts.first();
    let moves = parts.last().unwrap();

    let matrix = cargo.map(|c| build_matrix(c));

    let inverted_cargo = matrix.map(|m| invert_cargo(m)).unwrap();

    let mut stacks = build_stacks(inverted_cargo);

    for line in moves.lines() {
        let cmd = Command::from(line);
        stacks = move_stacks(stacks, &cmd);
    }

    Some(get_result(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let cargo = parts.first();
    let moves = parts.last().unwrap();

    let matrix = cargo.map(|c| build_matrix(c));

    let inverted_cargo = matrix.map(|m| invert_cargo(m)).unwrap();

    let mut stacks = build_stacks(inverted_cargo);

    for line in moves.lines() {
        let cmd = Command::from(line);
        stacks = move_stacks(stacks, &cmd);
    }

    Some(get_result(stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
