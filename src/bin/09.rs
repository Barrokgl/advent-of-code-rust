use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, iter};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
enum Quadrant {
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

fn parse_command(command: &str) -> Vec<Direction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<direction>R|U|L|D) (?P<number>\d+)").unwrap();
    }
    RE.captures(command)
        .map(|cap| {
            let direction = cap.name("direction").map(|m| m.as_str());
            let number = cap
                .name("number")
                .and_then(|n| n.as_str().parse::<usize>().ok())
                .unwrap_or(0);
            match direction {
                Some(dir) if dir == "R" => iter::repeat(Direction::Right).take(number).collect(),
                Some(dir) if dir == "L" => iter::repeat(Direction::Left).take(number).collect(),
                Some(dir) if dir == "U" => iter::repeat(Direction::Up).take(number).collect(),
                Some(dir) if dir == "D" => iter::repeat(Direction::Down).take(number).collect(),
                _ => vec![],
            }
        })
        .unwrap_or(vec![])
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_adjacent_to(&self, other: &Point) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy == 1 || (dx <= 1 && dy <= 1)
    }

    fn is_overlap_with(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn move_point(&self, dir: &Direction) -> Point {
        match dir {
            Direction::Right => Point::new(self.x + 1, self.y),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
        }
    }

    fn move_by_diagonal(&self, dir: &Quadrant) -> Point {
        match dir {
            Quadrant::UpRight => Point::new(self.x + 1, self.y + 1),
            Quadrant::UpLeft => Point::new(self.x - 1, self.y + 1),
            Quadrant::DownLeft => Point::new(self.x - 1, self.y - 1),
            Quadrant::DownRight => Point::new(self.x + 1, self.y - 1),
        }
    }

    fn get_quadrant_direction(&self, other: &Point) -> Quadrant {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        match (dx.is_positive(), dy.is_positive()) {
            (true, true) => Quadrant::UpRight,
            (true, false) => Quadrant::DownRight,
            (false, true) => Quadrant::UpLeft,
            (false, false) => Quadrant::DownLeft,
        }
    }

    fn is_same_plane(&self, other: &Point) -> bool {
        self.x == other.x || self.y == other.y
    }
}

fn simulate_rope(commands: Vec<Direction>) -> i32 {
    let head = Point::new(0, 0);
    let tail = Point::new(0, 0);
    let (result_set, _, _): (HashSet<Point>, Point, Point) =
        commands
            .iter()
            .fold((HashSet::new(), head, tail), |(mut set, head, tail), d| {
                let new_head = head.move_point(d);
                let new_tail =
                    match tail.is_adjacent_to(&new_head) || tail.is_overlap_with(&new_head) {
                        true => tail,
                        false => {
                            if tail.is_same_plane(&new_head) {
                                tail.move_point(d)
                            } else {
                                let dir = tail.get_quadrant_direction(&new_head);
                                tail.move_by_diagonal(&dir)
                            }
                        }
                    };
                set.insert(new_tail);

                (set, new_head, new_tail)
            });
    result_set.len() as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands: Vec<Direction> = input
        .lines()
        .map(|line| parse_command(line))
        .flatten()
        .collect();
    let number_of_points = simulate_rope(commands);

    Some(number_of_points as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
