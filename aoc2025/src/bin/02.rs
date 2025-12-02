use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let nums: Vec<(u64, u64)> = input
        .split(",")
        .map(|c| {
            c.split("-")
                .map(|x| x.trim())
                .map(|x| x.parse::<u64>().expect("Should be unsigned numbers"))
                .collect_tuple()
                .expect("Should be tuple of 2")
        })
        .collect_vec();
    let total = nums.iter().fold(0, |total, (first, second)| {
        // optimized for the use case
        let first_num_digits = first.ilog10() + 1;
        let second_num_digits = second.ilog10() + 1;
        let num_digits_diff = second_num_digits - first_num_digits;
        let mut start = *first;
        let mut curr_num_digits = first_num_digits;
        let mut total = total;
        for _ in 0..=num_digits_diff {
            if !curr_num_digits.is_multiple_of(2) {
                start = 10_u64.pow(curr_num_digits);
                curr_num_digits += 1;
                continue;
            }
            let end = if curr_num_digits == second_num_digits {
                *second
            } else {
                10_u64.pow(curr_num_digits) - 1
            };
            let mut start_top = start / 10_u64.pow(curr_num_digits / 2);
            let start_bottom = start - start_top * 10_u64.pow(curr_num_digits / 2);
            if start_bottom > start_top {
                // already past the possible repeat
                start_top += 1;
            }
            let mut end_top = end / 10_u64.pow(curr_num_digits / 2);
            let end_bottom = end - end_top * 10_u64.pow(curr_num_digits / 2);
            if end_bottom < end_top {
                // ends before the possible repeat
                end_top -= 1;
            }
            for i in start_top..=end_top {
                let invalid = i * 10_u64.pow(curr_num_digits / 2) + i;
                total += invalid;
            }
            if end < *second {
                start = 10_u64.pow(curr_num_digits + 1);
                curr_num_digits += 1;
            }
        }
        total
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums: Vec<(u64, u64)> = input
        .split(",")
        .map(|c| {
            c.split("-")
                .map(|x| x.trim())
                .map(|x| x.parse::<u64>().expect("Should be unsigned numbers"))
                .collect_tuple()
                .expect("Should be tuple of 2")
        })
        .collect_vec();
    let total = nums.iter().fold(0, |mut total, (first, second)| {
        for number in *first..=*second {
            let num_digits = number.ilog10() + 1;
            'second_loop: for i in 2..=num_digits {
                let valid = num_digits % i == 0;
                if !valid {
                    continue;
                }
                let check_num_digits = num_digits / i;
                let check_num = number / 10_u64.pow(num_digits - check_num_digits);
                let mut curr = number - check_num * 10_u64.pow(num_digits - check_num_digits);
                for j in 1..i {
                    let curr_num_digits = num_digits - check_num_digits - j * check_num_digits;
                    let check = curr / 10_u64.pow(curr_num_digits);
                    if check != check_num {
                        continue 'second_loop;
                    }
                    if (curr.ilog10() + 1) < curr_num_digits {
                        // there's a leading 0
                        continue 'second_loop;
                    }
                    curr -= check_num * 10_u64.pow(curr_num_digits);
                }
                total += number;
                break;
            }
        }
        total
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
