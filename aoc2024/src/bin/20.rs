use std::collections::VecDeque;

use aoc_utils::*;

advent_of_code::solution!(20);

#[cfg(not(test))]
const SAVE_CUTOFF: usize = 100;
#[cfg(test)]
const SAVE_CUTOFF: usize = 50;

fn find_path(start: Point, end: Point, map: &[Vec<char>]) -> Vec<Point> {
    let bounds = Bounds(map.len() - 1, map[0].len() - 1);
    let mut seen = vec![vec![false; bounds.1 + 1]; bounds.0 + 1];

    let mut queue = VecDeque::new();
    let mut path = Vec::new();

    seen[start.0][start.1] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        path.push(curr);
        if curr == end {
            break;
        }
        Dir::neighbors(curr, bounds).into_iter().for_each(|p| {
            if seen[p.0][p.1] {
                return;
            }
            seen[p.0][p.1] = true;
            if map[p.0][p.1] != '#' {
                queue.push_back(p);
            }
        });
    }
    path
}

fn find_cheats(path: &[Point], max_dist: usize) -> u64 {
    path.iter()
        .enumerate()
        .take(path.len() - SAVE_CUTOFF - 1)
        .fold(0, |mut acc, (i, p)| {
            path.iter()
                .enumerate()
                .skip(i + SAVE_CUTOFF + 2)
                .for_each(|(j, p2)| {
                    let d = dist(*p, *p2);
                    if j - i - d >= SAVE_CUTOFF && d <= max_dist {
                        acc += 1;
                    }
                });
            acc
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.c_map();
    let start = find_point(&map, 'S');
    let end = find_point(&map, 'E');
    let path = find_path(start, end, &map);
    Some(find_cheats(&path, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.c_map();
    let start = find_point(&map, 'S');
    let end = find_point(&map, 'E');
    let path = find_path(start, end, &map);
    Some(find_cheats(&path, 20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
