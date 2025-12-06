use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let total = input.trim().chars().fold(0, |total, c| match c {
        '(' => total + 1,
        ')' => total - 1,
        other => panic!("Shouldn't exist: {}", other),
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    input
        .trim()
        .chars()
        .find_position(|c| match c {
            '(' => {
                total += 1;
                false
            }
            ')' => {
                total -= 1;
                total < 0
            }
            other => panic!("Shouldn't exist: {}", other),
        })
        .map(|(i, _)| (i + 1) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
