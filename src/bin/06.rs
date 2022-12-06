use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let window_size: u32 = 4;
    let chars = input.chars().collect::<Vec<char>>();

    let uniq_seq = chars
        .windows(window_size.try_into().unwrap())
        .find(|window| HashSet::<char>::from_iter((window).iter().cloned()).len() == window.len())
        .map(|chars| chars.iter().collect::<String>());

    let uninq_seq_index = input
        .match_indices(&uniq_seq.unwrap())
        .find_map(|m| Some(m.0 as u32));
    uninq_seq_index.map(|n| n + window_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let window_size: u32 = 14;
    let chars = input.chars().collect::<Vec<char>>();

    let uniq_seq = chars
        .windows(window_size.try_into().unwrap())
        .find(|window| HashSet::<char>::from_iter((window).iter().cloned()).len() == window.len())
        .map(|chars| chars.iter().collect::<String>());

    let uninq_seq_index = input
        .match_indices(&uniq_seq.unwrap())
        .find_map(|m| Some(m.0 as u32));
    uninq_seq_index.map(|n| n + window_size)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
