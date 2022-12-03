fn convert_pick(pick: &str) -> &str {
    match pick {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!("cannot convert oponent pick"),
    }
}

fn get_point_for_pick(pick: &str) -> u32 {
    match pick {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("cannot get points for pick"),
    }
}

fn get_round_result(oponent_pick: &str, player_pick: &str) -> u32 {
    (match (oponent_pick, player_pick) {
        (x, y) if x == y => 3,
        ("A", "B") => 6,
        ("A", "C") => 0,
        ("B", "A") => 0,
        ("B", "C") => 6,
        ("C", "A") => 6,
        ("C", "B") => 0,
        (_, _) => panic!("cannot match round result"),
    }) + get_point_for_pick(player_pick)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |sum, line| {
        sum + get_round_result(&line[0..1], convert_pick(&line[2..3]))
    }))
}

fn pick_by_strategy<'a>(strategy: &'a str, oponent_pick: &'a str) -> &'a str {
    match (strategy, oponent_pick) {
        ("Y", x) => x,
        ("X", "A") => "C",
        ("X", "B") => "A",
        ("X", "C") => "B",
        ("Z", "A") => "B",
        ("Z", "B") => "C",
        ("Z", "C") => "A",

        _ => panic!("cannot pick by strategy"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |sum, line| {
        sum + get_round_result(&line[0..1], pick_by_strategy(&line[2..3], &line[0..1]))
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
