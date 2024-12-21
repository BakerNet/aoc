use aoc_utils::*;
use memoize::memoize;

advent_of_code::solution!(21);

fn keypad_loc(c: char) -> Point {
    match c {
        '7' => Point(0, 0),
        '8' => Point(0, 1),
        '9' => Point(0, 2),
        '4' => Point(1, 0),
        '5' => Point(1, 1),
        '6' => Point(1, 2),
        '1' => Point(2, 0),
        '2' => Point(2, 1),
        '3' => Point(2, 2),
        '0' => Point(3, 1),
        'A' => Point(3, 2),
        _ => panic!("unexpected char"),
    }
}

fn dirpad_loc(c: char) -> Point {
    match c {
        '^' => Point(0, 1),
        'A' => Point(0, 2),
        '<' => Point(1, 0),
        'v' => Point(1, 1),
        '>' => Point(1, 2),
        _ => panic!("unexpected char"),
    }
}

fn solve(num: &str, layers: u64) -> u64 {
    bot_moves_dirpad(0, layers, num.to_string())
}

#[memoize]
fn bot_moves_dirpad(layer: u64, max_layer: u64, prev: String) -> u64 {
    if layer > max_layer {
        return prev.len() as u64;
    }
    if prev.is_empty() {
        return 0;
    }
    let mut pos = if layer == 0 { Point(3, 2) } else { Point(0, 2) };
    let bad_pos = if layer == 0 { Point(3, 0) } else { Point(0, 0) };
    let mut ans = 0;
    prev.chars().for_each(|c| {
        let start = pos;
        let mut vpath = String::new();
        let mut hpath = String::new();
        let target = if layer == 0 {
            keypad_loc(c)
        } else {
            dirpad_loc(c)
        };
        while target.0 < pos.0 {
            vpath += "^";
            pos.0 -= 1;
        }
        while target.0 > pos.0 {
            vpath += "v";
            pos.0 += 1;
        }
        while target.1 > pos.1 {
            hpath += ">";
            pos.1 += 1;
        }
        while target.1 < pos.1 {
            hpath += "<";
            pos.1 -= 1;
        }
        let res1 = bot_moves_dirpad(layer + 1, max_layer, hpath.clone() + &vpath + "A");
        let res2 = bot_moves_dirpad(layer + 1, max_layer, vpath.clone() + &hpath + "A");
        if target.0 == bad_pos.0 && start.1 == bad_pos.1 {
            ans += res1;
        } else if target.1 == bad_pos.1 && start.0 == bad_pos.0 {
            ans += res2;
        } else if res1 < res2 {
            ans += res1;
        } else {
            ans += res2;
        }
    });
    ans
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input.mlines(|l| {
        let n = l[..l.len() - 1]
            .parse::<u64>()
            .expect("Should parse number");
        (l.to_string(), n)
    });
    Some(nums.into_iter().map(|(l, x)| x * solve(&l, 2)).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input.mlines(|l| {
        let n = l[..l.len() - 1]
            .parse::<u64>()
            .expect("Should parse number");
        (l.to_string(), n)
    });
    Some(
        nums.into_iter()
            .map(|(l, x)| x * solve(&l, 25))
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
