use aoc_utils::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .mlines(|line| {
            let len = line.len();
            let chars = line.as_bytes();
            let mut max = 11;
            for start in 0..(len - 1) {
                for end in (start + 1)..len {
                    let num = (chars[start] - 48) * 10 + (chars[end] - 48);
                    if num > max {
                        max = num;
                        if max == 99 {
                            return max;
                        }
                    }
                }
            }
            max
        })
        .into_iter()
        .map(|x| x as u64)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .mlines(|line| {
            let len = line.len();
            let chars = line.as_bytes();
            let mut final_num = 0_u64;
            let mut place = 0;
            // 12 batteries
            for digit in 0..12 {
                let mut current = 0;
                for loc in place..(len - (11 - digit)) {
                    if (chars[loc] - 48) > current {
                        current = chars[loc] - 48;
                        place = loc + 1;
                    }
                }
                final_num = final_num * 10 + current as u64;
            }
            final_num
        })
        .into_iter()
        .map(|x| x as u64)
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
