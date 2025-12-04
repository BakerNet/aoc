use std::collections::BTreeSet;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.c_map();
    let num = map
        .iter()
        .enumerate()
        .flat_map(|(x, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == '@')
                .map(|(y, _)| Point(x, y))
                .collect_vec()
        })
        .filter(|p| {
            DirExt::neighbors(*p, Bounds(map.len() - 1, map[0].len() - 1))
                .iter()
                .filter(|p2| map[p2.0][p2.1] == '@')
                .count()
                < 4
        })
        .count();
    Some(num as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input.c_map();
    let mut queue = map
        .iter()
        .enumerate()
        .flat_map(|(x, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == '@')
                .map(|(y, _)| Point(x, y))
                .collect_vec()
        })
        .collect::<BTreeSet<Point>>();
    let mut removed = 0;
    while !queue.is_empty() {
        let curr = queue.pop_first().unwrap();
        let neighbors = DirExt::neighbors(curr, Bounds(map.len() - 1, map[0].len() - 1));
        if neighbors.iter().filter(|p| map[p.0][p.1] == '@').count() < 4 {
            removed += 1;
            map[curr.0][curr.1] = '.';
            neighbors
                .into_iter()
                .filter(|p| map[p.0][p.1] == '@')
                .for_each(|p| {
                    queue.insert(p);
                });
        }
    }
    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
