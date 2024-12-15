use std::collections::{HashMap, VecDeque};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn handle_move(d: Dir, curr: &mut (usize, usize), map: &mut [Vec<char>]) {
    let mut temp = (curr.0, curr.1);
    match d {
        Dir::Up => match map[curr.0 - 1][curr.1] {
            '#' => {}
            '.' => {
                curr.0 -= 1;
            }
            'O' => {
                while temp.0 > 0 {
                    temp.0 -= 1;
                    if map[temp.0][temp.1] == '#' {
                        return;
                    }
                    if map[temp.0][temp.1] == '.' {
                        map[curr.0 - 1][curr.1] = '.';
                        map[temp.0][temp.1] = 'O';
                        curr.0 -= 1;
                        return;
                    }
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Down => match map[curr.0 + 1][curr.1] {
            '#' => {}
            '.' => {
                curr.0 += 1;
            }
            'O' => {
                while temp.0 < map.len() - 1 {
                    temp.0 += 1;
                    if map[temp.0][temp.1] == '#' {
                        return;
                    }
                    if map[temp.0][temp.1] == '.' {
                        map[curr.0 + 1][curr.1] = '.';
                        map[temp.0][temp.1] = 'O';
                        curr.0 += 1;
                        return;
                    }
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Left => match map[curr.0][curr.1 - 1] {
            '#' => {}
            '.' => {
                curr.1 -= 1;
            }
            'O' => {
                while temp.1 > 0 {
                    temp.1 -= 1;
                    if map[temp.0][temp.1] == '#' {
                        return;
                    }
                    if map[temp.0][temp.1] == '.' {
                        map[curr.0][curr.1 - 1] = '.';
                        map[temp.0][temp.1] = 'O';
                        curr.1 -= 1;
                        return;
                    }
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Right => match map[curr.0][curr.1 + 1] {
            '#' => {}
            '.' => {
                curr.1 += 1;
            }
            'O' => {
                while temp.1 < map[0].len() - 1 {
                    temp.1 += 1;
                    if map[temp.0][temp.1] == '#' {
                        return;
                    }
                    if map[temp.0][temp.1] == '.' {
                        map[curr.0][curr.1 + 1] = '.';
                        map[temp.0][temp.1] = 'O';
                        curr.1 += 1;
                        return;
                    }
                }
            }
            _ => panic!("unexpected char"),
        },
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.blocks();
    let mut map = parts[0].c_map();
    let moves = parts[1]
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            _ => None,
        })
        .collect_vec();
    let mut bot = map
        .iter()
        .enumerate()
        .find_map(|(row, v)| {
            v.iter()
                .enumerate()
                .find_map(|(col, c)| if *c == '@' { Some((row, col)) } else { None })
        })
        .expect("Sohuld find start");
    map[bot.0][bot.1] = '.';
    moves.into_iter().for_each(|d| {
        handle_move(d, &mut bot, &mut map);
    });
    Some(
        map.iter()
            .enumerate()
            .flat_map(|(row, v)| {
                v.iter().enumerate().filter_map(move |(col, c)| {
                    if *c == 'O' {
                        Some((100 * row + col) as u64)
                    } else {
                        None
                    }
                })
            })
            .sum::<u64>(),
    )
}

fn handle_move_p2(d: Dir, curr: &mut (usize, usize), map: &mut [Vec<char>]) {
    match d {
        Dir::Up => match map[curr.0 - 1][curr.1] {
            '#' => {}
            '.' => {
                curr.0 -= 1;
            }
            '[' | ']' => {
                if shift_boxes((curr.0 - 1, curr.1), d, map) {
                    curr.0 -= 1;
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Down => match map[curr.0 + 1][curr.1] {
            '#' => {}
            '.' => {
                curr.0 += 1;
            }
            '[' | ']' => {
                if shift_boxes((curr.0 + 1, curr.1), d, map) {
                    curr.0 += 1;
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Left => match map[curr.0][curr.1 - 1] {
            '#' => {}
            '.' => {
                curr.1 -= 1;
            }
            '[' | ']' => {
                if shift_boxes((curr.0, curr.1 - 1), d, map) {
                    curr.1 -= 1;
                }
            }
            _ => panic!("unexpected char"),
        },
        Dir::Right => match map[curr.0][curr.1 + 1] {
            '#' => {}
            '.' => {
                curr.1 += 1;
            }
            '[' | ']' => {
                if shift_boxes((curr.0, curr.1 + 1), d, map) {
                    curr.1 += 1;
                }
            }
            _ => panic!("unexpected char"),
        },
    }
}

fn shift_boxes(temp: (usize, usize), d: Dir, map: &mut [Vec<char>]) -> bool {
    if matches!(d, Dir::Right | Dir::Left) {
        simple_shift(temp, d, map)
    } else {
        complex_shift(temp, d, map)
    }
}

fn box_group_up(start: (usize, usize), d: Dir, map: &[Vec<char>]) -> HashMap<(usize, usize), char> {
    let mut queue = VecDeque::new();
    let mut boxes = HashMap::new();
    match map[start.0][start.1] {
        '[' => {
            queue.push_back((start.0, start.1));
            queue.push_back((start.0, start.1 + 1));
        }
        ']' => {
            queue.push_back((start.0, start.1));
            queue.push_back((start.0, start.1 - 1));
        }
        _ => panic!("unexpected char in group up"),
    }
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        boxes.insert(curr, map[curr.0][curr.1]);
        let check = if matches!(d, Dir::Up) {
            curr.0 - 1
        } else {
            curr.0 + 1
        };
        match map[check][curr.1] {
            '[' => {
                queue.push_back((check, curr.1));
                queue.push_back((check, curr.1 + 1));
            }
            ']' => {
                queue.push_back((check, curr.1));
                queue.push_back((check, curr.1 - 1));
            }
            _ => {}
        }
    }
    boxes
}

fn complex_shift(temp: (usize, usize), d: Dir, map: &mut [Vec<char>]) -> bool {
    match d {
        Dir::Up => {
            let boxes = box_group_up(temp, d, map);
            if boxes.keys().any(|p| map[p.0 - 1][p.1] == '#') {
                false
            } else {
                boxes.iter().for_each(|(p, _)| {
                    map[p.0][p.1] = '.';
                });
                boxes.into_iter().for_each(|(p, c)| {
                    map[p.0 - 1][p.1] = c;
                });
                true
            }
        }
        Dir::Down => {
            let boxes = box_group_up(temp, d, map);
            if boxes.keys().any(|p| map[p.0 + 1][p.1] == '#') {
                false
            } else {
                boxes.iter().for_each(|(p, _)| {
                    map[p.0][p.1] = '.';
                });
                boxes.into_iter().for_each(|(p, c)| {
                    map[p.0 + 1][p.1] = c;
                });
                true
            }
        }
        _ => panic!("Called complex_shift on Left or Right"),
    }
}

fn simple_shift(temp: (usize, usize), d: Dir, map: &mut [Vec<char>]) -> bool {
    let orig = temp;
    let mut temp = temp;
    match d {
        Dir::Left => {
            while temp.1 > 0 {
                temp.1 -= 1;
                if map[temp.0][temp.1] == '#' {
                    return false;
                }
                if map[temp.0][temp.1] == '.' {
                    while temp.1 < orig.1 {
                        map[temp.0][temp.1] = map[temp.0][temp.1 + 1];
                        temp.1 += 1;
                    }
                    map[orig.0][orig.1] = '.';
                    return true;
                }
            }
        }
        Dir::Right => {
            while temp.1 < map[0].len() - 1 {
                temp.1 += 1;
                if map[temp.0][temp.1] == '#' {
                    return false;
                }
                if map[temp.0][temp.1] == '.' {
                    while temp.1 > orig.1 {
                        map[temp.0][temp.1] = map[temp.0][temp.1 - 1];
                        temp.1 -= 1;
                    }
                    map[orig.0][orig.1] = '.';
                    return true;
                }
            }
        }
        _ => panic!("called simple shift with Up or Down"),
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = input.blocks();
    let mut map = parts[0].mlines(|l| {
        l.chars()
            .fold(Vec::with_capacity(l.len() * 2), |mut acc, c| {
                match c {
                    '#' => {
                        acc.push('#');
                        acc.push('#');
                    }
                    '.' => {
                        acc.push('.');
                        acc.push('.');
                    }
                    'O' => {
                        acc.push('[');
                        acc.push(']');
                    }
                    '@' => {
                        acc.push('@');
                        acc.push('.');
                    }
                    _ => panic!("unkown char"),
                }
                acc
            })
    });
    let moves = parts[1]
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            _ => None,
        })
        .collect_vec();
    let mut bot = map
        .iter()
        .enumerate()
        .find_map(|(row, v)| {
            v.iter()
                .enumerate()
                .find_map(|(col, c)| if *c == '@' { Some((row, col)) } else { None })
        })
        .expect("Sohuld find start");
    map[bot.0][bot.1] = '.';
    moves.into_iter().for_each(|d| {
        handle_move_p2(d, &mut bot, &mut map);
    });
    Some(
        map.iter()
            .enumerate()
            .flat_map(|(row, v)| {
                v.iter().enumerate().filter_map(move |(col, c)| {
                    if *c == '[' {
                        Some((100 * row + col) as u64)
                    } else {
                        None
                    }
                })
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(618));
    }
}
