use itertools::Itertools;

advent_of_code::solution!(4);

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn valid_xmas_dirs(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<Self> {
        let mut dirs = Vec::with_capacity(8);
        if row >= 3 {
            dirs.push(Direction::Up);
            if col >= 3 {
                dirs.push(Direction::UpLeft);
            }
            if col <= max_col - 3 {
                dirs.push(Direction::UpRight);
            }
        }
        if row <= max_row - 3 {
            dirs.push(Direction::Down);
            if col >= 3 {
                dirs.push(Direction::DownLeft);
            }
            if col <= max_col - 3 {
                dirs.push(Direction::DownRight);
            }
        }
        if col >= 3 {
            dirs.push(Direction::Left);
        }
        if col <= max_col - 3 {
            dirs.push(Direction::Right);
        }
        dirs
    }

    fn valid_x_dirs(
        row: usize,
        col: usize,
        max_row: usize,
        max_col: usize,
    ) -> Option<Vec<(Direction, Direction)>> {
        if row == 0 || row == max_row || col == 0 || col == max_col {
            None
        } else {
            Some(vec![
                (Direction::UpLeft, Direction::DownRight),
                (Direction::UpRight, Direction::DownLeft),
            ])
        }
    }

    fn next(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Self::Up => (row - 1, col),
            Self::UpRight => (row - 1, col + 1),
            Self::Right => (row, col + 1),
            Self::DownRight => (row + 1, col + 1),
            Self::Down => (row + 1, col),
            Self::DownLeft => (row + 1, col - 1),
            Self::Left => (row, col - 1),
            Self::UpLeft => (row - 1, col - 1),
        }
    }
}

fn find_xmas(graph: &[Vec<char>], row: usize, col: usize) -> usize {
    let dirs = Direction::valid_xmas_dirs(row, col, graph.len() - 1, graph[0].len() - 1);
    dirs.into_iter()
        .filter(|d| {
            let mi = d.next(row, col);
            if graph[mi.0][mi.1] != 'M' {
                return false;
            }
            let ai = d.next(mi.0, mi.1);
            if graph[ai.0][ai.1] != 'A' {
                return false;
            }
            let xi = d.next(ai.0, ai.1);
            if graph[xi.0][xi.1] != 'S' {
                return false;
            }
            true
        })
        .count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let xmases = graph
        .iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(col, c)| {
                    if *c != 'X' {
                        None
                    } else {
                        Some(find_xmas(&graph, row, col) as u64)
                    }
                })
                .sum::<u64>()
        })
        .sum();
    Some(xmases)
}

fn find_x_mas(graph: &[Vec<char>], row: usize, col: usize) -> bool {
    if let Some(x_dirs) = Direction::valid_x_dirs(row, col, graph.len() - 1, graph[0].len() - 1) {
        let xms = x_dirs
            .iter()
            .filter(|(d1, d2)| {
                let rc1 = d1.next(row, col);
                let rc2 = d2.next(row, col);
                (graph[rc1.0][rc1.1] == 'M' && graph[rc2.0][rc2.1] == 'S')
                    || (graph[rc2.0][rc2.1] == 'M' && graph[rc1.0][rc1.1] == 'S')
            })
            .count();
        xms == 2
    } else {
        false
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let xmases = graph
        .iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|&(col, c)| {
                    if *c != 'A' {
                        false
                    } else {
                        find_x_mas(&graph, row, col)
                    }
                })
                .count() as u64
        })
        .sum();
    Some(xmases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
