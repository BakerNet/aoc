use std::{
    collections::{HashMap, HashSet},
    ops::BitXor,
};

use aoc_utils::*;

fn transform_num(num: u64) -> u64 {
    let mut num = num;
    num = num.bitxor(num * 64) % 16777216;
    num = num.bitxor(num / 32) % 16777216;
    num.bitxor(num * 2048) % 16777216
}

fn secret_number(num: u64, times: usize) -> u64 {
    let mut num = num;
    for _ in 0..times {
        num = transform_num(num);
    }
    num
}

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .mlines(|l| {
                let x = ufroms(l);
                secret_number(x, 2000)
            })
            .into_iter()
            .sum::<u64>(),
    )
}

fn secret_number_with_seqcount(num: u64, times: usize, counts: &mut HashMap<[i8; 4], u64>) -> u64 {
    let mut inner_counts: HashSet<[i8; 4]> = HashSet::new();
    let mut num = num;
    let mut prev_arry = [0; 4];
    for i in 0..times {
        let prev = (num % 10) as i8;
        num = transform_num(num);
        let curr = (num % 10) as i8;

        if i < 3 {
            prev_arry[i + 1] = curr - prev;
            continue;
        }
        prev_arry.rotate_left(1);
        prev_arry[3] = curr - prev;
        if !inner_counts.contains(&prev_arry) {
            inner_counts.insert(prev_arry);
            counts
                .entry(prev_arry)
                .and_modify(|x| *x += curr as u64)
                .or_insert(curr as u64);
        }
    }
    num
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counts = HashMap::new();
    for l in input.lines() {
        let num = ufroms(l);
        secret_number_with_seqcount(num, 2000, &mut counts);
    }
    Some(
        counts
            .into_values()
            .max()
            .expect("There should be max value"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_123() {
        let mut counts = HashMap::new();
        secret_number_with_seqcount(123, 10, &mut counts);
        assert_eq!(counts.get(&[-3, 6, -1, -1]), Some(&4));
        assert_eq!(counts.get(&[6, -1, -1, 0]), Some(&4));
        assert_eq!(counts.get(&[-1, -1, 0, 2]), Some(&6));
    }
}
