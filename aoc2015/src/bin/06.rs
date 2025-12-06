use aoc_utils::*;
use regex::Regex;

advent_of_code::solution!(6);

fn parse_input(input: &str) -> Vec<(String, usize, usize, usize, usize)> {
    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    input.regex_mlines(re, |c| {
        let op = c.get(1).expect("Should have first match").as_str();
        let aa: usize = c.get_num(2);
        let ab: usize = c.get_num(3);
        let ba: usize = c.get_num(4);
        let bb: usize = c.get_num(5);
        (op.to_string(), aa, ab, ba, bb)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lights = vec![vec![false; 1000]; 1000];
    let steps = parse_input(input);
    steps.into_iter().for_each(|(op, aa, ab, ba, bb)| {
        for row in lights.iter_mut().take(ba + 1).skip(aa) {
            for bulb in row.iter_mut().take(bb + 1).skip(ab) {
                match op.as_str() {
                    "turn on" => *bulb = true,
                    "turn off" => *bulb = false,
                    "toggle" => *bulb = !*bulb,
                    other => panic!("Shouldn't exist: {}", other),
                }
            }
        }
    });
    let mut total = 0;
    for row in lights.into_iter() {
        for item in row.into_iter() {
            if item {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lights = vec![vec![0_i16; 1000]; 1000];
    let steps = parse_input(input);
    steps.into_iter().for_each(|(op, aa, ab, ba, bb)| {
        for row in lights.iter_mut().take(ba + 1).skip(aa) {
            for bulb in row.iter_mut().take(bb + 1).skip(ab) {
                match op.as_str() {
                    "turn on" => *bulb += 1,
                    "turn off" => *bulb = 0.max(*bulb - 1),
                    "toggle" => *bulb += 2,
                    other => panic!("Shouldn't exist: {}", other),
                }
            }
        }
    });
    let mut total: u64 = 0;
    for row in lights.into_iter() {
        for item in row.into_iter() {
            total += item as u64;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998996));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1001996));
    }
}
