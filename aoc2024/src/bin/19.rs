use std::collections::{HashMap, HashSet};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(19);

fn is_possible<'a>(
    s: &'a str,
    towels: &HashSet<&'a str>,
    max_len: usize,
    seen: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(b) = seen.get(s) {
        return *b;
    }
    if s.is_empty() {
        return true;
    }
    let max = max_len.min(s.len());
    for e in 1..max + 1 {
        let substr = &s[0..e];
        if towels.contains(substr) {
            if is_possible(&s[e..], towels, max_len, seen) {
                seen.insert(s, true);
                seen.insert(&s[e..], true);
                return true;
            } else {
                seen.insert(&s[e..], false);
            }
        }
    }
    seen.insert(s, false);
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = input.blocks();
    let towels = blocks[0].split(", ").collect::<HashSet<&str>>();
    let max_len = towels
        .iter()
        .map(|s| s.len())
        .max()
        .expect("There should be a max");
    let patterns = blocks[1].lines().collect_vec();
    let mut memo = HashMap::new();
    Some(
        patterns
            .into_iter()
            .filter(|s| is_possible(s, &towels, max_len, &mut memo))
            .count() as u64,
    )
}

fn possible_ways<'a>(
    s: &'a str,
    towels: &HashSet<&'a str>,
    max_len: usize,
    seen: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(b) = seen.get(s) {
        return *b;
    }
    if s.is_empty() {
        return 1;
    }
    let max = max_len.min(s.len());
    let mut count = 0;
    for e in 1..max + 1 {
        let substr = &s[0..e];
        if towels.contains(substr) {
            count += possible_ways(&s[e..], towels, max_len, seen);
        }
    }
    seen.insert(s, count);
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = input.blocks();
    let towels = blocks[0].split(", ").collect::<HashSet<&str>>();
    let max_len = towels
        .iter()
        .map(|s| s.len())
        .max()
        .expect("There should be a max");
    let patterns = blocks[1].lines().collect_vec();
    let mut memo = HashMap::new();
    Some(
        patterns
            .into_iter()
            .map(|s| possible_ways(s, &towels, max_len, &mut memo))
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
