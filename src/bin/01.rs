use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let elfs: Vec<_> = input.lines().collect();
    let mut buff: Vec<u32> = vec![];
    let mut result: Option<u32> = None;

    for i in elfs {
        match i {
            "" => {
                let sum: u32 = buff.iter().sum();
                buff = vec![];
                result = result.or(Some(0)).map(|a| max(a, sum))
            }
            num => buff.push(num.parse::<u32>().unwrap()),
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elfs: Vec<_> = input.lines().collect();
    let mut buff: Vec<u32> = vec![];
    let mut result: Vec<u32> = vec![];
    elfs.push("");

    for i in elfs {
        match i {
            "" => {
                let sum: u32 = buff.iter().sum();
                buff = vec![];
                result.push(sum);
            }
            num => buff.push(num.parse::<u32>().unwrap()),
        }
    }

    result.sort();

    Some(result.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
