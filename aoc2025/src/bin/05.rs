use std::{cmp::Ordering, ops::Range};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_ranges(input: &str) -> (Vec<Range<u64>>, &str) {
    let blocks = input.blocks();
    let ranges = blocks[0];
    let ids = blocks[1];
    let ranges = ranges.mlines(|s| {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .expect("should be a first")
            .parse::<u64>()
            .expect("first should be int");
        let end = parts
            .next()
            .expect("should be a second")
            .parse::<u64>()
            .expect("second should be int")
            + 1;
        Range { start, end }
    });
    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_ranges(input);
    // could be faster by sorting the ranges and then binary search, but eh...
    let is_in_range = move |x: &u64| ranges.iter().any(|r| r.contains(x));
    Some(
        ids.lines()
            .map(|s| s.parse::<u64>().expect("ids should be ints"))
            .filter(is_in_range)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_ranges(input);
    Some(
        ranges
            .into_iter()
            .sorted_by(|a, b| match Ord::cmp(&a.start, &b.start) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ord::cmp(&a.end, &b.end),
                Ordering::Greater => Ordering::Greater,
            })
            .fold((0, 0), |(last_seen, total), r| {
                let end = if r.end < last_seen {
                    return (last_seen, total);
                } else {
                    r.end
                };
                let start = if r.start < last_seen {
                    last_seen
                } else {
                    r.start
                };
                (end, total + end - start)
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
