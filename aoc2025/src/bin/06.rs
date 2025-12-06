use itertools::Itertools;

advent_of_code::solution!(6);

fn split_input(input: &str) -> (&str, Vec<&str>) {
    let lines = input.lines().collect_vec();
    // trim because of part 2 final coumn width calculation
    let ops_line = &lines[lines.len() - 1].trim();
    let num_lines = lines[..lines.len() - 1].to_vec();
    (ops_line, num_lines)
}

fn calc_total(ops: &[&str], nums: &[Vec<u64>]) -> u64 {
    nums.iter()
        .enumerate()
        .fold(vec![0; ops.len()], |mut totals, (i, v)| {
            let t = &mut totals[i];
            v.iter().for_each(|x| match ops[i] {
                _ if *t == 0 => *t = *x,
                "*" => *t *= *x,
                "+" => *t += *x,
                other => panic!("Should exist: {}", other),
            });
            totals
        })
        .iter()
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ops_line, num_lines) = split_input(input);
    let ops = ops_line.split_whitespace().collect_vec();
    let ops_count = ops.len();
    let mut nums = vec![vec![0; num_lines.len()]; ops_count];
    num_lines.iter().enumerate().for_each(|(i, &s)| {
        s.split_whitespace()
            .enumerate()
            .for_each(|(col, x)| nums[col][i] = x.parse::<u64>().expect("should be ints"))
    });
    Some(calc_total(&ops, &nums))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ops_line, num_lines) = split_input(input);
    let ops = ops_line.split_whitespace().collect_vec();
    let ops_count = ops.len();
    let mut col_numbers = vec![0; ops_count];
    // each op is the first index in the number column for its set of numbers
    // use this to get the count of columns (numbers) in the number column
    ops_line.chars().skip(1).fold(0, |col, c| match c {
        '*' | '+' => col + 1,
        ' ' => {
            col_numbers[col] += 1;
            col
        }
        other => panic!("Shouldn't exist: {}", other),
    });
    // last col_width can't be known from ops row alone
    let max_width = num_lines
        .iter()
        .map(|s| s.len())
        .max()
        .expect("Sum should exist");
    col_numbers[ops_count - 1] = max_width - ops_line.len() + 1;

    let mut nums = col_numbers.iter().map(|x| vec![0; *x]).collect_vec();
    num_lines.iter().for_each(|&s| {
        s.chars().fold((0, 0), |(col_idx, number_idx), c| {
            if number_idx >= col_numbers[col_idx] {
                // we are in-between number columns
                return (col_idx + 1, 0);
            }
            let update_num = &mut nums[col_idx][number_idx];
            if let Some(x) = c.to_digit(10) {
                *update_num = *update_num * 10 + x as u64;
            }
            (col_idx, number_idx + 1)
        });
    });
    Some(calc_total(&ops, &nums))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
