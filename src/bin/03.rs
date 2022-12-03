fn get_char_code(letter: Vec<char>) -> u8 {
    letter.first().map_or(0, |letter| {
        let n = *letter as u8;
        if letter.is_ascii_lowercase() {
            n - 96
        } else if letter.is_ascii_uppercase() {
            (n - 64) + 26
        } else {
            panic!("cannot get char code")
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |sum, rucksack| {
        let (first_part, second_part) = (
            &rucksack[0..(rucksack.len() / 2)],
            &rucksack[rucksack.len() / 2..rucksack.len()],
        );
        let mut result: Vec<char> = first_part
            .chars()
            .filter(|c| second_part.contains(&c.to_string()))
            .collect();
        result.dedup();
        sum + get_char_code(result) as u32
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<_> = input.lines().collect();
    Some(rucksacks.chunks(3).fold(0, |sum, chunk| {
        let mut chunk_sacks = Vec::from(chunk);
        chunk_sacks.sort();

        match chunk_sacks.last() {
            Some(l) => {
                let mut result: Vec<char> = l
                    .chars()
                    .filter(|c| {
                        chunk_sacks[0..chunk_sacks.len() - 1]
                            .iter()
                            .all(|sack| sack.contains(&c.to_string()))
                    })
                    .collect();
                result.dedup();
                sum + get_char_code(result) as u32
            }
            None => panic!("cannot find longest str"),
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);

        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
