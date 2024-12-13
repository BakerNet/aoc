use itertools::Itertools;
use tinyvec::{array_vec, ArrayVec};

advent_of_code::solution!(10);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Should be digits"))
                .collect_vec()
        })
        .collect_vec()
}

fn neighbors(point: Point, max_row: usize, max_col: usize) -> ArrayVec<[Point; 8]> {
    let mut neighbors = array_vec!([Point; 8]);

    let row = point.0;
    let col = point.1;
    if col > 0 {
        neighbors.push(Point(row, col - 1));
    }
    if col < max_col {
        neighbors.push(Point(row, col + 1));
    }
    if row > 0 {
        neighbors.push(Point(row - 1, col));
    }
    if row < max_row {
        neighbors.push(Point(row + 1, col));
    }
    neighbors
}

fn valid_trails(curr: Point, map: &[Vec<u32>], seen: &mut [Vec<bool>]) -> usize {
    seen[curr.0][curr.1] = true;
    let curr_val = map[curr.0][curr.1];
    if map[curr.0][curr.1] == 9 {
        return 1;
    }
    let ns = neighbors(curr, map.len() - 1, map[0].len() - 1);
    ns.into_iter()
        .filter_map(|p| {
            if map[p.0][p.1] == curr_val + 1 && !seen[p.0][p.1] {
                Some(valid_trails(p, map, seen))
            } else {
                None
            }
        })
        .sum()
}

fn valid_trails_all(curr: Point, map: &[Vec<u32>]) -> usize {
    let curr_val = map[curr.0][curr.1];
    if map[curr.0][curr.1] == 9 {
        return 1;
    }
    let ns = neighbors(curr, map.len() - 1, map[0].len() - 1);
    ns.into_iter()
        .filter_map(|p| {
            if map[p.0][p.1] == curr_val + 1 {
                Some(valid_trails_all(p, map))
            } else {
                None
            }
        })
        .sum()
}

fn solve(map: &[Vec<u32>], part_2: bool) -> u64 {
    map.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(|(col, _)| {
                    let n = if !part_2 {
                        let mut seen = map
                            .iter()
                            .map(|v| v.iter().map(|_| false).collect_vec())
                            .collect_vec();
                        valid_trails(Point(row, col), map, &mut seen)
                    } else {
                        valid_trails_all(Point(row, col), map)
                    };
                    n as u64
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    Some(solve(&map, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    Some(solve(&map, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
