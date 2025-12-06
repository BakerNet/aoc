
advent_of_code::solution!(4);

fn solve(input: &str, num_zeroes: usize) -> Option<u64> {
    let zeroes = "0".repeat(num_zeroes);
    let input = input.trim();
    let mut suffix = 1;
    loop {
        let digest = md5::compute(format!("{input}{suffix}").as_bytes());
        if format!("{:x}", digest).starts_with(&zeroes) {
            return Some(suffix);
        }
        suffix += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 5)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "slow"]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[ignore = "slow"]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
