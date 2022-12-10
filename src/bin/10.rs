#[derive(Clone, Copy, Debug)]
enum Command {
    Addx(i32),
    Noop,
}
use std::iter;

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
    let mut x: i32 = 1;
    let mut result: u32 = 0;
    let cycles_probe = vec![20, 60, 100, 140, 180, 220];
    for (i, cmd) in commands.iter().enumerate() {
        let cycle_number = i + 1;

        if cycles_probe.contains(&cycle_number) {
            let singal_power: u32 = (x * cycle_number as i32).try_into().unwrap_or(0);
            result = result + singal_power
        }

        match cmd {
            Command::Addx(n) => x = x + n,
            Command::Noop => (),
        };
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let commands: Vec<Command> = input
        .lines()
        .map(parse_command)
        .fold(Vec::new(), |acc, cmd| match cmd {
            Command::Addx(_) => vec![acc, vec![Command::Noop, cmd]].concat(),
            Command::Noop => vec![acc, vec![cmd]].concat(),
        });
    let mut res: Vec<String> = Vec::new();
    let mut x: i32 = 1;

    for (i, cmds_list) in commands.chunks(40).enumerate() {
        let mut crt_row: Vec<&str> = iter::repeat(".").take(40).collect();
        for (pixel, cmd) in cmds_list.iter().enumerate() {
            if (x - pixel as i32).abs() <= 1 {
                crt_row[pixel] = "#";
            }

            match cmd {
                Command::Addx(n) => x = x + n,
                Command::Noop => (),
            };
        }
        res.push(crt_row.join(""));
    }
    println!("{}", res.join("\n"));

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
