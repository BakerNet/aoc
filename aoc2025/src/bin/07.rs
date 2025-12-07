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
    // zero alocations for ZOOM ZOOM
    let bytes = input.as_bytes();
    let width = input.find('\n').unwrap();
    let mut map_counts = vec![0; width];
    let s_loc = width / 2; // starting location of 'S'
    map_counts[s_loc] = 1;
    let mut skip = s_loc;
    for row in bytes.chunks(width + 1).step_by(2).skip(1) {
        skip -= 1;
        for c in skip..width - skip - 1 {
            let curr = row[c];
            if curr == b'^' {
                map_counts[c - 1] += map_counts[c];
                map_counts[c + 1] += map_counts[c];
                map_counts[c] = 0;
            }
        }
    }
    Some(map_counts.iter().sum())
}

pub fn part_two_original(input: &str) -> Option<u64> {
    let map = input.c_map();
    let width = map[0].len();
    let mut map_counts = vec![0; width];
    let mut new_map_counts = vec![0; width];
    let mut skip = width / 2;
    for row in map.iter().step_by(2) {
        for c in skip..row.len() - skip {
            let prev = map_counts[c];
            let curr = row[c];
            match (prev, curr) {
                (_, 'S') => new_map_counts[c] = 1,
                (x, '.') if x > 0 => {
                    new_map_counts[c] += map_counts[c];
                }
                (x, '^') if x > 0 => {
                    new_map_counts[c - 1] += map_counts[c];
                    new_map_counts[c + 1] += map_counts[c];
                }
                _ => (),
            }
        }
        std::mem::swap(&mut map_counts, &mut new_map_counts);
        if skip != 0 {
            skip -= 1;
            new_map_counts.iter_mut().for_each(|x| *x = 0);
        }
    }
    let timelines = map_counts.iter().sum();
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
