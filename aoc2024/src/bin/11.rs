use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn split_num(num: u64, len: u32) -> (u64, u64) {
    let tens = 10_u64.pow(len / 2);
    (num / tens, num % tens)
}

fn blink(curr: u64, count: u8, to: u8, seen: &mut HashMap<(u64, u8), u64>) -> u64 {
    if count == to {
        return 1;
    }
    if let Some(x) = seen.get(&(curr, count)) {
        return *x;
    }
    let res = if curr == 0 {
        blink(1, count + 1, to, seen)
    } else {
        let len = curr.checked_ilog10().unwrap_or(0) + 1;
        if len % 2 == 0 {
            let (first, second) = split_num(curr, len);
            blink(first, count + 1, to, seen) + blink(second, count + 1, to, seen)
        } else {
            blink(curr * 2024, count + 1, to, seen)
        }
    };
    seen.insert((curr, count), res);
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let start = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Should be numbers"))
        .collect_vec();
    let mut seen = HashMap::<(u64, u8), u64>::new();
    Some(start.into_iter().map(|x| blink(x, 0, 25, &mut seen)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let start = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Should be numbers"))
        .collect_vec();
    let mut seen = HashMap::<(u64, u8), u64>::new();
    Some(start.into_iter().map(|x| blink(x, 0, 75, &mut seen)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
