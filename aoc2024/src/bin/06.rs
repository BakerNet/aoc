use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }

    fn next(
        &self,
        row: usize,
        col: usize,
        max_row: usize,
        max_col: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row < max_row {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col < max_col {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map = input.lines().map(|v| v.chars().collect_vec()).collect_vec();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(row, v)| {
            v.iter()
                .find_position(|c| **c == '^')
                .map(|(col, _)| (row, col))
        })
        .expect("Should find guard");
    (map, start)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut map, start) = parse_input(input);
    map[start.0][start.1] = '-';
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut dir = Direction::Up;
    let mut pos = start;
    while let Some((r, c)) = dir.next(pos.0, pos.1, max_row, max_col) {
        if map[r][c] == '#' {
            dir = dir.turn();
        } else {
            map[r][c] = '-';
            pos = (r, c);
        }
    }
    Some(map.into_iter().flatten().filter(|c| *c == '-').count() as u64)
}

fn has_loop(map: &[Vec<char>], start: (usize, usize)) -> bool {
    let mut seen = vec![vec![Vec::<Direction>::new(); map[0].len()]; map.len()];
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut dir = Direction::Up;
    let mut pos = start;
    while let Some((r, c)) = dir.next(pos.0, pos.1, max_row, max_col) {
        if map[r][c] == '#' {
            dir = dir.turn();
        } else {
            if seen[r][c].contains(&dir) {
                return true;
            }
            seen[r][c].push(dir);
            pos = (r, c);
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut map, start) = parse_input(input);
    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let c = map[row][col];
            if c == '#' || c == '^' {
                continue;
            }
            map[row][col] = '#';
            if has_loop(&map, start) {
                count += 1;
            }
            map[row][col] = '.';
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
