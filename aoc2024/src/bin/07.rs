use itertools::Itertools;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let s1 = l.split_once(":").expect("Should be able to split on :");
            let target = s1.0.parse::<u64>().expect("Should be number 1");
            let nums =
                s1.1.split_whitespace()
                    .map(|x| x.parse::<u64>().expect("Should be number 2"))
                    .collect_vec();
            (target, nums)
        })
        .collect_vec()
}

enum Op {
    Add,
    Mult,
    Conc,
}

impl Op {
    fn do_op(&self, x: u64, y: u64) -> u64 {
        match self {
            Self::Add => x + y,
            Self::Mult => x * y,
            Self::Conc => format!("{x}{y}")
                .parse::<u64>()
                .expect("Should be able to concatenate nums"),
        }
    }
}

fn is_valid(target: u64, curr: u64, remaining: &[u64], ops: &[Op]) -> bool {
    if curr > target {
        return false;
    }
    if remaining.is_empty() {
        if curr == target {
            return true;
        }
        return false;
    }
    ops.iter()
        .any(|op| is_valid(target, op.do_op(curr, remaining[0]), &remaining[1..], ops))
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    Some(
        lines
            .into_iter()
            .filter(|(t, ns)| is_valid(*t, 0, ns, &[Op::Add, Op::Mult]))
            .map(|(t, _)| t)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    Some(
        lines
            .into_iter()
            .filter(|(t, ns)| is_valid(*t, 0, ns, &[Op::Add, Op::Mult, Op::Conc]))
            .map(|(t, _)| t)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
