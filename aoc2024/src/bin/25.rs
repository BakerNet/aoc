use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(25);

fn heights(input: &str) -> (bool, Vec<usize>) {
    let grid = input.c_map();
    let is_lock = grid[0][0] == '#';
    let heights = (0..grid[0].len())
        .map(|col| {
            grid.iter()
                .map(|row| row[col])
                .filter(|&c| c == '#')
                .count()
        })
        .collect_vec();
    (is_lock, heights)
}

pub fn part_one(input: &str) -> Option<u64> {
    let max = input.blocks()[0].lines().count();
    let (locks, keys) = input.mblocks(heights).into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut a, mut b), (is_lock, heights)| {
            if is_lock {
                a.push(heights);
            } else {
                b.push(heights);
            }
            (a, b)
        },
    );
    Some(locks.into_iter().fold(0, |acc, lhs| {
        acc + keys.iter().fold(0, |acc2, khs| {
            if lhs.iter().zip(khs.iter()).any(|(l, k)| l + k > max) {
                acc2
            } else {
                acc2 + 1
            }
        })
    }))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
