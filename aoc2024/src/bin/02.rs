use itertools::Itertools;

advent_of_code::solution!(2);

fn process_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<u64>().expect("Should all be numbers"))
                .collect_vec()
        })
        .collect_vec()
}

fn is_increasing(v: &[u64]) -> bool {
    let one = v[0] < v[1];
    let two = v[1] < v[2];
    let three = v[2] < v[3];
    one && two || (one || two) && three
}

fn is_bad(x: u64, y: u64, increasing: bool) -> bool {
    let (x, y) = if increasing { (x, y) } else { (y, x) };
    x >= y || y - x > 3
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = process_input(input);
    Some(
        lines
            .into_iter()
            .filter(|v| {
                let increasing = is_increasing(v);
                !v.windows(2).any(|w| is_bad(w[0], w[1], increasing))
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = process_input(input);
    Some(
        lines
            .into_iter()
            .filter(|v| {
                let up = is_increasing(v);
                let bad = v.windows(2).find_position(|w| is_bad(w[0], w[1], up));
                if let Some((i, _)) = bad {
                    let mut nv = v.clone();
                    nv.remove(i);
                    let mut nv2 = v.clone();
                    nv2.remove(i + 1);
                    !nv.windows(2).any(|w| is_bad(w[0], w[1], up))
                        || !nv2.windows(2).any(|w| is_bad(w[0], w[1], up))
                } else {
                    true
                }
            })
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
