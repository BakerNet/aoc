use aoc_utils::*;

advent_of_code::solution!(3);

fn find_max_num(num_digits: usize, line: &str) -> u64 {
    let len = line.len();
    let chars = line.as_bytes();
    let mut final_num = 0_u64;
    let mut place = 0;
    for digit in 0..num_digits {
        let mut current = 0;
        let place_clone = place;
        #[allow(clippy::needless_range_loop)]
        for loc in place_clone..(len - (num_digits - 1 - digit)) {
            // 48 == ascii '0'
            if (chars[loc] - 48) > current {
                current = chars[loc] - 48;
                place = loc + 1;
            }
            if current == 9 {
                break;
            }
        }
        final_num = final_num * 10 + current as u64;
    }
    final_num
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input.mlines(|line| find_max_num(2, line)).into_iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .mlines(|line| find_max_num(12, line))
        .into_iter()
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
