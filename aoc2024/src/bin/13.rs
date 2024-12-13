use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Clone, Copy, Debug)]
struct Claw {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl Claw {
    fn solve(self) -> Option<(u64, u64)> {
        let fa = (self.a.0 as f64, self.a.1 as f64);
        let fb = (self.b.0 as f64, self.b.1 as f64);
        let det = (fa.0 * fb.1) - (fb.0 * fa.1);
        if det == 0.0 {
            return None;
        }
        let fprize = ((self.prize.0 as f64), (self.prize.1 as f64));

        let x = ((fb.1 * fprize.0) - (fb.0 * fprize.1)) / det;
        let y = ((fa.1 * fprize.0) - (fa.0 * fprize.1)) / -det;
        if x < 0.0 || y < 0.0 || x.fract() != 0.0 || y.fract() != 0.0 {
            return None;
        }
        Some((x as u64, y as u64))
    }
}

fn parse_input(line: &str) -> Claw {
    let parts = line.lines().collect_vec();
    let re = Regex::new(r"^Button [A|B]: X\+(\d+), Y\+(\d+)$").expect("Regex should compile");
    let re2 = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").expect("Regex2 should compile");
    let a_caps = re
        .captures(parts[0])
        .expect("Should have valid format button line");
    let a = (
        a_caps
            .get(1)
            .expect("Should get button X val")
            .as_str()
            .parse::<u64>()
            .expect("X capture should be int"),
        a_caps
            .get(2)
            .expect("Should get button Y val")
            .as_str()
            .parse::<u64>()
            .expect("Y capture should be int"),
    );
    let b_caps = re
        .captures(parts[1])
        .expect("Should have valid format button line");
    let b = (
        b_caps
            .get(1)
            .expect("Should get button X val")
            .as_str()
            .parse::<u64>()
            .expect("X capture should be int"),
        b_caps
            .get(2)
            .expect("Should get button Y val")
            .as_str()
            .parse::<u64>()
            .expect("Y capture should be int"),
    );
    let prize_caps = re2
        .captures(parts[2])
        .expect("Should have valid format prize line");
    let prize = (
        prize_caps
            .get(1)
            .expect("Should get prize X val")
            .as_str()
            .parse::<u64>()
            .expect("X capture should be int"),
        prize_caps
            .get(2)
            .expect("Should get prize Y val")
            .as_str()
            .parse::<u64>()
            .expect("Y capture should be int"),
    );
    Claw { a, b, prize }
}

pub fn part_one(input: &str) -> Option<u64> {
    let total = input
        .split("\n\n")
        .map(parse_input)
        .filter_map(|c| c.solve().map(|(a, b)| a * 3 + b))
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total = input
        .split("\n\n")
        .map(parse_input)
        .map(|c| Claw {
            a: c.a,
            b: c.b,
            prize: (c.prize.0 + 10000000000000, c.prize.1 + 10000000000000),
        })
        .filter_map(|c| c.solve().map(|(a, b)| a * 3 + b))
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
