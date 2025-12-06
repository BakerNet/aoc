use std::collections::HashSet;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Dir> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            '^' => Dir::Up,
            'v' => Dir::Down,
            other => panic!("Shouldn't exist: {other}"),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let dirs = parse_input(input);
    let mut visited = HashSet::new();
    let mut curr = IPoint(0, 0);
    visited.insert(curr);
    dirs.iter().for_each(|d| {
        curr = d.inext(curr);
        visited.insert(curr);
    });
    Some(visited.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let dirs = parse_input(input);
    let mut visited = HashSet::new();
    let mut curr_santa = IPoint(0, 0);
    let mut curr_ranta = IPoint(0, 0);
    visited.insert(curr_santa);
    dirs.iter().enumerate().for_each(|(i, d)| {
        if i.is_multiple_of(2) {
            curr_santa = d.inext(curr_santa);
            visited.insert(curr_santa);
        } else {
            curr_ranta = d.inext(curr_ranta);
            visited.insert(curr_ranta);
        }
    });
    Some(visited.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }
}
