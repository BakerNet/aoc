use std::collections::VecDeque;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(18);

#[cfg(not(test))]
const NUMBER_OF_BLOCKS: usize = 1024;
#[cfg(test)]
const NUMBER_OF_BLOCKS: usize = 12;
#[cfg(not(test))]
const SIZE_OF_MAP: usize = 71;
#[cfg(test)]
const SIZE_OF_MAP: usize = 7;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Loc {
    Empty,
    Block,
}

fn find_shortest_path_len(
    curr: Point,
    len: u64,
    map: &[Vec<Loc>],
    seen: &mut [Vec<bool>],
) -> Option<u64> {
    let bounds = Bounds(map.len() - 1, map[0].len() - 1);
    seen[curr.0][curr.1] = true;

    let mut queue = VecDeque::new();
    queue.push_back((curr, len));

    while !queue.is_empty() {
        let (curr, len) = queue.pop_front().unwrap();
        if curr == Point(bounds.0, bounds.1) {
            return Some(len);
        }
        Dir::neighbors(curr, bounds).into_iter().for_each(|p| {
            if !seen[p.0][p.1] && map[p.0][p.1] != Loc::Block {
                seen[p.0][p.1] = true;
                queue.push_back((p, len + 1));
            }
        })
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.mlines(|l| {
        l.split_once(",")
            .map(|(x, y)| {
                Point(
                    x.parse::<usize>().expect("Should be number"),
                    y.parse::<usize>().expect("Should be number"),
                )
            })
            .expect("Should split")
    });
    let mut map = vec![vec![Loc::Empty; SIZE_OF_MAP]; SIZE_OF_MAP];
    let mut seen = vec![vec![false; SIZE_OF_MAP]; SIZE_OF_MAP];
    points
        .into_iter()
        .take(NUMBER_OF_BLOCKS)
        .for_each(|p| map[p.0][p.1] = Loc::Block);
    find_shortest_path_len(Point(0, 0), 0, &map, &mut seen)
}

fn find_shortest_path(
    curr: Point,
    len: u64,
    map: &[Vec<Loc>],
    seen: &mut [Vec<bool>],
) -> Option<Vec<Point>> {
    let bounds = Bounds(map.len() - 1, map[0].len() - 1);
    seen[curr.0][curr.1] = true;

    let mut queue = VecDeque::new();
    queue.push_back((vec![curr], len));

    while !queue.is_empty() {
        let (curr_v, len) = queue.pop_front().unwrap();
        let curr = curr_v.iter().last().unwrap();
        if *curr == Point(bounds.0, bounds.1) {
            return Some(curr_v);
        }
        Dir::neighbors(*curr, bounds).into_iter().for_each(|p| {
            if !seen[p.0][p.1] && map[p.0][p.1] != Loc::Block {
                seen[p.0][p.1] = true;
                let mut new_v = curr_v.clone();
                new_v.push(p);
                queue.push_back((new_v, len + 1));
            }
        })
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let points = input.mlines(|l| {
        l.split_once(",")
            .map(|(x, y)| {
                Point(
                    x.parse::<usize>().expect("Should be number"),
                    y.parse::<usize>().expect("Should be number"),
                )
            })
            .expect("Should split")
    });
    let mut map = vec![vec![Loc::Empty; SIZE_OF_MAP]; SIZE_OF_MAP];
    let mut curr_short_path = map
        .iter()
        .enumerate()
        .flat_map(|(row, v)| v.iter().enumerate().map(move |(col, _)| Point(row, col)))
        .collect_vec();
    points
        .iter()
        .take(NUMBER_OF_BLOCKS)
        .for_each(|p| map[p.0][p.1] = Loc::Block);
    points.into_iter().skip(NUMBER_OF_BLOCKS).find_map(|p| {
        map[p.0][p.1] = Loc::Block;
        if curr_short_path.contains(&p) {
            let mut seen = vec![vec![false; SIZE_OF_MAP]; SIZE_OF_MAP];
            if let Some(v) = find_shortest_path(Point(0, 0), 0, &map, &mut seen) {
                curr_short_path = v;
                None
            } else {
                Some(format!("{},{}", p.0, p.1))
            }
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
