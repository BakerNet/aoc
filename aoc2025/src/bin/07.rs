use aoc_utils::*;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = input.c_map();
    let mut splits = 0;
    for r in 1..map.len() {
        for c in 0..map[r].len() {
            let prev = map[r - 1][c];
            let curr = map[r][c];
            match (prev, curr) {
                ('S' | '|', '.') => map[r][c] = '|',
                ('S' | '|', '^') => {
                    splits += 1;
                    map[r][c - 1] = '|';
                    map[r][c + 1] = '|';
                }
                _ => (),
            }
        }
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.c_map();
    let mut map_counts = vec![vec![0; map[0].len()]; map.len()];
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let prev = map_counts[if r > 0 { r - 1 } else { 0 }][c];
            let curr = map[r][c];
            match (prev, curr) {
                (_, 'S') => map_counts[r][c] = 1,
                (x, '.') if x > 0 => {
                    map_counts[r][c] += map_counts[r - 1][c];
                }
                (x, '^') if x > 0 => {
                    map_counts[r][c - 1] += map_counts[r - 1][c];
                    map_counts[r][c + 1] += map_counts[r - 1][c];
                }
                _ => (),
            }
        }
    }
    let timelines = map_counts[map_counts.len() - 1].iter().sum();
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
