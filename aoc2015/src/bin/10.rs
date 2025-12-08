advent_of_code::solution!(10);

fn run_n_times(input: &str, n: usize) -> String {
    let mut curr = input.trim().to_string();
    let mut new = String::new();
    for _ in 0..n {
        let mut chars = curr.chars();
        let mut curr_char = chars.next().expect("Should never have empty string");
        let mut curr_count = 1;
        for c in chars {
            if c != curr_char {
                new += &format!("{curr_count}{curr_char}");
                curr_char = c;
                curr_count = 1;
            } else {
                curr_count += 1;
            }
        }
        curr = new + &format!("{curr_count}{curr_char}");
        new = String::new();
    }
    curr
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = run_n_times(input, 40);
    Some(res.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = run_n_times(input, 50);
    Some(res.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(86710));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227900));
    }
}
