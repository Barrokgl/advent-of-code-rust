use lazy_static::lazy_static;

use regex::{Match, Regex};

struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn from_line(segment_str: &str) -> (Segment, Segment) {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<first>\d+)-(?P<second>\d+),(?P<third>\d+)-(?P<fourth>\d+)")
                    .unwrap();
        }

        RE.captures(segment_str)
            .map(|cap| {
                (
                    Segment {
                        x: parse_capture(cap.name("first")).unwrap(),
                        y: parse_capture(cap.name("second")).unwrap(),
                    },
                    Segment {
                        x: parse_capture(cap.name("third")).unwrap(),
                        y: parse_capture(cap.name("fourth")).unwrap(),
                    },
                )
            })
            .unwrap()
    }

    fn check_subsegment(&self, other: &Segment) -> bool {
        if (other.x >= self.x && other.x <= self.y) && (other.y <= self.y && other.y >= self.x) {
            true
        } else {
            false
        }
    }

    fn check_overlap(&self, other: &Segment) -> bool {
        if (other.x >= self.x && other.x <= self.y) || (other.y <= self.y && other.y >= self.x) {
            true
        } else {
            false
        }
    }
}

fn parse_capture(cap: Option<Match>) -> Option<i32> {
    cap.and_then(|x| x.as_str().parse::<i32>().ok())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |sum, line| {
        let (first, second) = Segment::from_line(line);

        if first.check_subsegment(&second) || second.check_subsegment(&first) {
            sum + 1
        } else {
            sum
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |sum, line| {
        let (first, second) = Segment::from_line(line);

        if first.check_overlap(&second) || second.check_overlap(&first) {
            sum + 1
        } else {
            sum
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
