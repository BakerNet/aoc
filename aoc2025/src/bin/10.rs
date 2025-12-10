use aoc_utils::*;
use itertools::Itertools;
use z3::{ast::Int, Optimize, SatResult};

advent_of_code::solution!(10);

fn parse_lights(input: &str) -> u64 {
    let mut lights = 0;
    let s = input.trim_start_matches('[').trim_end_matches(']');
    for (i, c) in s.as_bytes().iter().enumerate() {
        if *c == b'#' {
            lights |= 1 << i;
        }
    }
    lights
}

fn parse_button(input: &str) -> u64 {
    let mut xor = 0;
    let s = input
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',');
    for c in s.map(|c| c.as_bytes()) {
        assert!(c.len() == 1);
        let d = c[0] - b'0';
        xor |= 1 << d;
    }
    xor
}

fn solve_part1(target: u64, buttons: &[u64]) -> Option<u64> {
    for count in 1..=buttons.len() {
        for v in buttons.iter().permutations(count) {
            let mut curr = 0;
            v.into_iter().for_each(|x| curr ^= x);
            if curr == target {
                return Some(count as u64);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    // gonna use bits for everything because xor can work here
    let lines = input.mlines(|s| {
        let mut i = s.split_whitespace();
        let lights = i.next().map(parse_lights).expect("Didn't find lights");
        // don't need joltages for part 1
        let _ = i.next_back().expect("Didn't find joltages");
        let buttons = i.map(parse_button).collect_vec();
        (lights, buttons)
    });
    Some(
        lines
            .iter()
            .map(|(target, buttons)| {
                solve_part1(*target, buttons).expect("Should find solution for machine")
            })
            .sum(),
    )
}

fn solve_part2(buttons: &[Vec<usize>], joltages: &[u64]) -> u64 {
    let opt = Optimize::new();

    // target_counts = number of times to press each button (what we're solving for)
    let target_counts: Vec<Int> = (0..buttons.len())
        .map(|i| Int::new_const(format!("p{i}")))
        .collect();

    // Constraint: (sum of target_counts[i] for all i where pos in buttons[i]) == joltages[pos]
    for (pos, &jolt) in joltages.iter().enumerate() {
        let terms: Vec<_> = buttons
            .iter()
            .enumerate()
            .filter(|(_, b)| b.contains(&pos))
            .map(|(i, _)| &target_counts[i])
            .collect();

        let sum = Int::add(&terms);
        opt.assert(&sum.eq(&Int::from_u64(jolt)));
    }

    // Constraint: t >= 0 (can't press a button negative times)
    let zero = Int::from_u64(0);
    for t in &target_counts {
        opt.assert(&t.ge(&zero));
    }

    // Minimize sum of target_counts
    let target_count_refs: Vec<_> = target_counts.iter().collect();
    let total = Int::add(&target_count_refs);
    opt.minimize(&total);

    // Solve and extract result
    assert_eq!(opt.check(&[]), SatResult::Sat);
    let model = opt.get_model().unwrap();

    target_counts
        .iter()
        .map(|v| model.eval(v, true).unwrap().as_u64().unwrap())
        .sum()
}

fn parse_buttons_part2(input: &str) -> Vec<usize> {
    // remove () before split
    input
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|x| x.parse().expect("Button item should be int"))
        .collect_vec()
}

fn parse_joltages(input: &str) -> Vec<u64> {
    // remove {} before split
    input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|x| x.parse().expect("Button item should be int"))
        .collect_vec()
}

pub fn part_two(input: &str) -> Option<u64> {
    // gonna use bits for everything because xor can work here
    let lines = input.mlines(|s| {
        let mut i = s.split_whitespace();
        // don't need lights for part 2
        let _ = i.next().expect("Didn't find lights");
        let joltages = i
            .next_back()
            .map(parse_joltages)
            .expect("Didn't find joltages");
        let buttons = i.map(parse_buttons_part2).collect_vec();
        (buttons, joltages)
    });
    Some(lines.iter().map(|(b, j)| solve_part2(b, j)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
