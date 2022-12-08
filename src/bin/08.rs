use std::cmp::max;
use std::collections::HashSet;

fn build_matrix(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| s != &"")
                .map(|s| s.parse::<i32>().ok())
                .flatten()
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>()
}

fn is_edge_located(x: usize, y: usize, matrix_size: usize) -> bool {
    match (x, y) {
        (x, y) if x == 0 || y == 0 || x == matrix_size - 1 || y == matrix_size - 1 => true,
        _ => false,
    }
}

fn traverse(
    x: usize,
    y: usize,
    f: &dyn Fn(usize, usize) -> Option<(usize, usize)>,
    matrix: &Vec<Vec<i32>>,
    tree_size: &i32,
) -> bool {
    if let Some((new_x, new_y)) = f(x, y) {
        let curr_tree_smaller = matrix
            .get(new_y)
            .and_then(|row| row.get(new_x))
            .map(|x| x < tree_size)
            .unwrap_or(false);
        let is_edge = is_edge_located(new_x, new_y, matrix.len());
        if !curr_tree_smaller || (is_edge && !curr_tree_smaller) {
            false
        } else if is_edge && curr_tree_smaller {
            true
        } else {
            traverse(new_x, new_y, f, matrix, tree_size)
        }
    } else {
        false
    }
}

fn traverse_sum(
    x: usize,
    y: usize,
    f: &dyn Fn(usize, usize) -> Option<(usize, usize)>,
    matrix: &Vec<Vec<i32>>,
    tree_size: &i32,
    sum: i32,
) -> i32 {
    if let Some((new_x, new_y)) = f(x, y) {
        let new_sum = sum + 1;
        let curr_tree_smaller = matrix
            .get(new_y)
            .and_then(|row| row.get(new_x))
            .map(|x| x < tree_size)
            .unwrap_or(false);
        let is_edge = is_edge_located(new_x, new_y, matrix.len());
        if !curr_tree_smaller || (is_edge && !curr_tree_smaller) || (is_edge && curr_tree_smaller) {
            new_sum
        } else {
            traverse_sum(new_x, new_y, f, matrix, tree_size, new_sum)
        }
    } else {
        sum
    }
}
fn traverse_tree(matrix: &Vec<Vec<i32>>, tree_x: usize, tree_y: usize) -> bool {
    let tree_size = matrix.get(tree_y).and_then(|row| row.get(tree_x)).unwrap();

    let directions: Vec<Box<dyn Fn(usize, usize) -> Option<(usize, usize)>>> = vec![
        Box::new(move |x: usize, y: usize| y.checked_sub(1).map(|y| (x, y))),
        Box::new(move |x: usize, y: usize| x.checked_add(1).map(|x| (x, y))),
        Box::new(move |x: usize, y: usize| y.checked_add(1).map(|y| (x, y))),
        Box::new(move |x: usize, y: usize| x.checked_sub(1).map(|x| (x, y))),
    ];

    directions.iter().fold(false, |acc, fun| {
        if acc {
            acc
        } else {
            traverse(tree_x, tree_y, fun, matrix, tree_size)
        }
    })
}

fn traverse_tree_sum(matrix: &Vec<Vec<i32>>, tree_x: usize, tree_y: usize) -> i32 {
    let tree_size = matrix.get(tree_y).and_then(|row| row.get(tree_x)).unwrap();

    let directions: Vec<Box<dyn Fn(usize, usize) -> Option<(usize, usize)>>> = vec![
        Box::new(move |x: usize, y: usize| y.checked_sub(1).map(|y| (x, y))),
        Box::new(move |x: usize, y: usize| x.checked_add(1).map(|x| (x, y))),
        Box::new(move |x: usize, y: usize| y.checked_add(1).map(|y| (x, y))),
        Box::new(move |x: usize, y: usize| x.checked_sub(1).map(|x| (x, y))),
    ];

    directions.iter().fold(1, |acc, fun| {
        acc * max(1, traverse_sum(tree_x, tree_y, fun, matrix, tree_size, 0))
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = build_matrix(input);
    let matrix_size = matrix.len();
    let mut result_set: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_edge_located(x, y, matrix_size) {
                result_set.insert((x, y));
            } else {
                let tree_visible = traverse_tree(&matrix, x, y);
                if tree_visible {
                    result_set.insert((x, y));
                }
            }
        }
    }

    Some(result_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = build_matrix(input);
    let matrix_size = matrix.len();
    let mut max_n: u32 = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !is_edge_located(x, y, matrix_size) {
                let trees_n = traverse_tree_sum(&matrix, x, y);
                max_n = max(max_n, trees_n as u32);
            }
        }
    }

    Some(max_n)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
