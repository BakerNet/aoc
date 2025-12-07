use std::collections::{HashMap, HashSet};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(9);

fn solve(input: &str) -> (u64, u64) {
    let mut pair_dist = HashMap::<UPair<&str>, u64>::new();
    let mut locs = HashSet::<&str>::new();
    input.lines().for_each(|s| {
        let mut parts = s.split_whitespace();
        let first = parts.next().expect("No first loc");
        let _ = parts.next();
        let second = parts.next().expect("No second loc");
        let _ = parts.next();
        let dist = parts
            .next()
            .expect("No dist val")
            .parse::<u64>()
            .expect("Dist should be int");
        pair_dist.insert(UPair::new(first, second), dist);
        locs.insert(first);
        locs.insert(second);
    });
    let mut shortest = u64::MAX;
    let mut longest = u64::MIN;
    for path in locs.iter().permutations(locs.len()) {
        let len = path
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| {
                pair_dist
                    .get(&UPair::new(*a, *b))
                    .expect("Should find pair in dists")
            })
            .sum();
        if len < shortest {
            shortest = len;
        }
        if len > longest {
            longest = len;
        }
    }
    (shortest, longest)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
