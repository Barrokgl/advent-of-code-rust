#[derive(Clone, Copy, Debug)]
enum Command {
    Addx(i32),
    Noop,
}

fn parse_command(line: &str) -> Command {
    match line {
        s if s.starts_with("addx") => Command::Addx(
            s.split(" ")
                .last()
                .and_then(|n| n.parse::<i32>().ok())
                .unwrap_or(0),
        ),
        _ => Command::Noop,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands: Vec<Command> = input
        .lines()
        .map(parse_command)
        .fold(Vec::new(), |acc, cmd| match cmd {
            Command::Addx(_) => vec![acc, vec![Command::Noop, cmd]].concat(),
            Command::Noop => vec![acc, vec![cmd]].concat(),
        });
    println!("commands: {:?}", commands);
    let mut x: i32 = 1;
    let mut result: u32 = 0;
    let cycles_probe = vec![20, 60, 100, 140, 180, 220];
    for (i, cmd) in commands.iter().enumerate() {
        let cycle_number = i + 1;
        println!("before cycle n {}, x: {}", cycle_number, x);

        if cycles_probe.contains(&cycle_number) {
            let singal_power: u32 = (x * cycle_number as i32).try_into().unwrap_or(0);
            result = result + singal_power
        }

        match cmd {
            Command::Addx(n) => x = x + n,
            Command::Noop => (),
        };
        println!("after cycle n {}, x: {}", cycle_number, x);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
