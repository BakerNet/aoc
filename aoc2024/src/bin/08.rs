use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i64, i64);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Bounds(i64, i64);

fn in_bounds(p: Point, bounds: Bounds) -> bool {
    p.0 >= 0 && p.0 <= bounds.0 && p.1 >= 0 && p.1 <= bounds.1
}

fn antinode(p1: Point, p2: Point, bounds: Bounds) -> Vec<Point> {
    let x_dist = p2.0 - p1.0;
    let y_dist = p2.1 - p1.1;
    let first = Point(p1.0 - x_dist, p1.1 - y_dist);
    let second = Point(p2.0 + x_dist, p2.1 + y_dist);
    let mut ans = Vec::new();
    if in_bounds(first, bounds) {
        ans.push(first);
    }
    if in_bounds(second, bounds) {
        ans.push(second);
    }
    ans
}

fn antinode_ext(p1: Point, p2: Point, bounds: Bounds) -> Vec<Point> {
    let x_dist = p2.0 - p1.0;
    let y_dist = p2.1 - p1.1;

    let mut ans = Vec::new();

    ans.push(p1);
    ans.push(p2);

    let mut first = Point(p1.0 - x_dist, p1.1 - y_dist);
    while in_bounds(first, bounds) {
        ans.push(first);
        first = Point(first.0 - x_dist, first.1 - y_dist);
    }

    let mut second = Point(p2.0 + x_dist, p2.1 + y_dist);
    while in_bounds(second, bounds) {
        ans.push(second);
        second = Point(second.0 + x_dist, second.1 + y_dist);
    }
    ans
}

fn parse_input(input: &str) -> (Bounds, Vec<Vec<Point>>) {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut sets = HashMap::<&char, Vec<Point>>::new();
    map.iter().enumerate().for_each(|(row, v)| {
        v.iter().enumerate().for_each(|(col, c)| {
            if *c == '.' {
                return;
            }
            sets.entry(c)
                .and_modify(|v| {
                    v.push(Point(row as i64, col as i64));
                })
                .or_insert(vec![Point(row as i64, col as i64)]);
        });
    });
    (
        Bounds(map.len() as i64 - 1, map[0].len() as i64 - 1),
        sets.into_values().collect_vec(),
    )
}

fn solve<F>(bounds: Bounds, sets: Vec<Vec<Point>>, processor: F) -> u64
where
    F: Fn(Point, Point, Bounds) -> Vec<Point>,
{
    let mut antinodes = HashSet::new();
    sets.into_iter().for_each(|v| {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let an = processor(v[i], v[j], bounds);
                for v in an.into_iter() {
                    antinodes.insert(v);
                }
            }
        }
    });
    antinodes.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let (bounds, sets) = parse_input(input);
    Some(solve(bounds, sets, antinode))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (bounds, sets) = parse_input(input);
    Some(solve(bounds, sets, antinode_ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
