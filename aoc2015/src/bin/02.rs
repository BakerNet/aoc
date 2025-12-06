use aoc_utils::*;
use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let totals = input.trim().regex_mlines(re, |parts| {
        let length = parts.get_num::<u64>(1);
        let width = parts.get_num::<u64>(2);
        let height = parts.get_num::<u64>(3);
        let side1 = length * width;
        let side2 = width * height;
        let side3 = length * height;
        let min_side = side1.min(side2).min(side3);
        let total = 2 * side1 + 2 * side2 + 2 * side3;
        total + min_side
    });
    Some(totals.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let totals = input.trim().regex_mlines(re, |parts| {
        let length = parts.get_num::<u64>(1);
        let width = parts.get_num::<u64>(2);
        let height = parts.get_num::<u64>(3);
        let (first_small, second_small) = if length < width {
            (length, width.min(height))
        } else {
            (width, length.min(height))
        };
        let perimeter = first_small * 2 + second_small * 2;
        let ribbon = length * width * height;
        ribbon + perimeter
    });
    Some(totals.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
