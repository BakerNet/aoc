use aoc_utils::*;

advent_of_code::solution!(8);

fn solve(input: &str, part2: bool) -> u64 {
    input
        .mlines(move |s| {
            let mut in_ecape = false;
            let mut count_chars = 0;
            for c in s.chars() {
                match (c, in_ecape) {
                    ('\\', false) => {
                        in_ecape = true;
                    }
                    ('\\', true) => {
                        count_chars += if part2 { 2 } else { 1 };
                        in_ecape = false;
                    }
                    ('"', true) => {
                        count_chars += if part2 { 2 } else { 1 };
                        in_ecape = false;
                    }
                    ('x', true) => {
                        count_chars += if part2 { 1 } else { 3 };
                        in_ecape = false;
                    }
                    _ => (),
                }
            }
            count_chars + if part2 { 4 } else { 2 }
        })
        .iter()
        .sum::<usize>() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
