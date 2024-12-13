use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<[u64; 2]>> {
    let r = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").expect("Regex should compile");
    input
        .lines()
        .map(|s| {
            r.captures_iter(s)
                .map(|c| {
                    let nums: [u64; 2] = c
                        .extract()
                        .1
                        .map(|i| i.parse::<u64>().expect("All captures should be numbers"));
                    nums
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|v| v.into_iter().map(|[x, y]| x * y).sum::<u64>())
            .sum(),
    )
}

fn parse_input2(input: &str) -> Vec<Vec<[u64; 2]>> {
    // empty capture groups for do/don't because Captures::extract requires consistent number of
    // captures
    let r = Regex::new(r"don't\(\)()()|do\(\)()()|mul\(([\d]{1,3}),([\d]{1,3})\)")
        .expect("Regex should compile");
    let mut active = true;
    input
        .lines()
        .map(|s| {
            r.captures_iter(s)
                .filter_map(|c| {
                    let items = c.extract();
                    if items.0 == "don't()" {
                        active = false;
                        return None;
                    } else if items.0 == "do()" {
                        active = true;
                        return None;
                    }
                    if !active {
                        return None;
                    }
                    let nums: [u64; 2] = items
                        .1
                        .map(|i| i.parse::<u64>().expect("All captures should be numbers"));
                    Some(nums)
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input2(input)
            .into_iter()
            .map(|v| v.into_iter().map(|[x, y]| x * y).sum::<u64>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
