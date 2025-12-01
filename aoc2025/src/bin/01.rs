use aoc_utils::*;
use regex::Regex;

advent_of_code::solution!(1);

static START: i64 = 50;

#[derive(Debug, Copy, Clone)]
enum RotDir {
    L,
    R,
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    dir: RotDir,
    count: i64,
}

fn rotations(input: &str) -> Vec<Rotation> {
    let re: Regex = Regex::new(r"(L|R)(\d+)").expect("Should be valid regex");
    input.regex_mlines(re, |c| Rotation {
        dir: if c.get(1).map_or("", |m| m.as_str()) == "L" {
            RotDir::L
        } else {
            RotDir::R
        },
        count: c.get_num::<i64>(2),
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = rotations(input);
    Some(
        rotations
            .iter()
            .fold((START, 0), |(curr, zcount), rot| {
                let count = if rot.count >= 100 {
                    rot.count % 100
                } else {
                    rot.count
                };
                let new = match rot.dir {
                    RotDir::L => curr - count,
                    RotDir::R => curr + count,
                };
                let new = match new {
                    n if n < 0 => n + 100,
                    n if n > 99 => n - 100,
                    n => n,
                };
                if new == 0 {
                    return (new, zcount + 1);
                }
                (new, zcount)
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = rotations(input);
    Some(
        rotations
            .iter()
            .fold((START, 0), |(curr, zcount), rot| {
                let (count, zcount) = if rot.count >= 100 {
                    (rot.count % 100, zcount + (rot.count / 100) as u64)
                } else {
                    (rot.count, zcount)
                };
                let new = match rot.dir {
                    RotDir::L => curr - count,
                    RotDir::R => curr + count,
                };
                match new {
                    0 => (0, zcount + 1),
                    n if n < 0 => (n + 100, if curr != 0 { zcount + 1 } else { zcount }),
                    n if n > 99 => (n - 100, zcount + 1),
                    n => (n, zcount),
                }
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
