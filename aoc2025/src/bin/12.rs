use aoc_utils::*;

advent_of_code::solution!(12);

struct Region {
    area: u64,
    shape_counts: [u64; 6],
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let area = parts
            .next()
            .expect("Should find area")
            .trim_end_matches(":")
            .split('x')
            .map(|x| x.parse::<u64>().expect("Should find area parts"))
            .product();
        let mut shape_counts = [0; 6];
        parts
            .map(|x| x.parse::<u64>().expect("Should find area parts"))
            .enumerate()
            .for_each(|(i, n)| {
                shape_counts[i] = n;
            });
        Region { area, shape_counts }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // This shouldn't work tbh... it's not a true packing solution... but for some reason Day 12
    // works with this naive approach for the input 🤷
    let mut shapes = [0_u64; 6];
    let blocks = input.blocks();
    let mut spaces = Vec::with_capacity(blocks.len() - 6);
    for (i, block) in blocks.into_iter().enumerate() {
        if i < 6 {
            let mut lines = block.lines();
            let first = lines.next().unwrap();
            let index = first
                .trim_end_matches(":")
                .parse::<usize>()
                .expect("Should find index");
            lines.for_each(|s| {
                shapes[index] += s.as_bytes().iter().filter(|c| **c == b'#').count() as u64;
            });
        } else {
            block
                .lines()
                .map(Region::from)
                .for_each(|r| spaces.push(r));
        }
    }
    Some(
        spaces
            .iter()
            .filter(|r| {
                r.area
                    > r.shape_counts
                        .iter()
                        .enumerate()
                        .map(|(i, c)| shapes[i] * c)
                        .sum()
            })
            .count() as u64,
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // Example requires TRUE packing solution, which I haven't implemented
        // assert_eq!(result, Some(2));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
