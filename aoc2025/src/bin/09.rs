#![feature(get_disjoint_mut_helpers)]
use core::slice::GetDisjointMutIndex;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(9);

fn area(a: Point, b: Point) -> u64 {
    (a.0.abs_diff(b.0) + 1) as u64 * (a.1.abs_diff(b.1) + 1) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = input.mlines(|s| {
        s.split(',')
            .map(|x| x.parse::<u64>().expect("Should be num"))
            .collect_tuple::<(u64, u64)>()
            .map(|(x, y)| Point(x as usize, y as usize))
            .expect("Should be 2 nums")
    });
    tiles
        .into_iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| dist(*a, *b))
        .next_back()
        .map(|(a, b)| area(a, b))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tiles = input.mlines(|s| {
        s.split(',')
            .map(|x| x.parse::<u64>().expect("Should be num"))
            .collect_tuple::<(u64, u64)>()
            .map(|(x, y)| Point(x as usize, y as usize))
            .expect("Should be 2 nums")
    });
    // add first to end to make sure windows will pick up every line
    tiles.push(tiles[0]);
    for (a, b) in tiles
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| dist(**a, **b))
        .rev()
    {
        let x_range = a.0.min(b.0) + 1..a.0.max(b.0);
        let y_range = a.1.min(b.1) + 1..a.1.max(b.1);
        if tiles
            .iter()
            .any(|p| x_range.contains(&p.0) && y_range.contains(&p.1))
        {
            // if there's a point inside, it's definitely not a complete square
            continue;
        }
        if !is_in_poly(Point((a.0 + b.0) / 2, (a.1 + b.1) / 2), &tiles) {
            // if the center isn't in the poly, def not a complete square
            continue;
        }
        if has_intersection_line(*a, *b, &tiles) {
            // if there are lines cutting through the internals of the square...
            // not a square
            continue;
        }
        return Some(area(*a, *b));
    }
    None
}

fn has_intersection_line(a: Point, b: Point, tiles: &[Point]) -> bool {
    // have to use inclusive range here to check against inclusive ranges below
    // otherwise, should have simplified to a.0.min(b.0) + 1..a.0.max(b.0);
    let x_range = a.0.min(b.0) + 1..=a.0.max(b.0) - 1;
    let y_range = a.1.min(b.1) + 1..=a.1.max(b.1) - 1;
    for v in tiles.windows(2) {
        let start = v[0];
        let end = v[1];

        if start.0 == end.0 && x_range.contains(&start.0) {
            // we are checking vertical line
            let other_y_range = start.1.min(end.1)..=start.1.max(end.1);
            if y_range.is_overlapping(&other_y_range) && y_range != other_y_range {
                return true;
            }
        }
        if start.1 == end.1 && y_range.contains(&start.1) {
            // we are checking horizontal line
            let other_x_range = start.0.min(end.0)..=start.0.max(end.0);
            if x_range.is_overlapping(&other_x_range) && y_range != other_x_range {
                return true;
            }
        }
    }
    false
}

fn is_in_poly(p: Point, tiles: &[Point]) -> bool {
    // use the Odd-Even rule to check inside poly
    let mut intersections = 0;
    for v in tiles.windows(2) {
        let start = v[0];
        let end = v[1];

        if start.1 == end.1 && p.1 == start.1 {
            // we might be on a horizontal line - if so, not in poly
            let x_range = start.0.min(end.0)..=start.0.max(end.0);
            if x_range.contains(&p.0) {
                return false;
            }
            continue;
        }

        if start.0 != end.0 {
            // only check vertical lines
            continue;
        }
        if start.0 >= p.0 {
            // only check lines to the left
            continue;
        }
        let y_range = start.1.min(end.1)..=start.1.max(end.1);
        if y_range.contains(&p.1) {
            intersections += 1;
        }
    }
    intersections % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
