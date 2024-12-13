use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use tinyvec::{array_vec, ArrayVec};

advent_of_code::solution!(12);

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn left(&self) -> Self {
        Point(self.0 - 1, self.1)
    }

    fn down(&self) -> Self {
        Point(self.0, self.1 - 1)
    }
}

fn neighbors(point: Point, c: char, map: &[Vec<char>]) -> ArrayVec<[Point; 4]> {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut neighbors = array_vec!([Point; 4]);

    let row = point.0;
    let col = point.1;
    if col > 0 && map[row][col - 1] == c {
        neighbors.push(Point(row, col - 1));
    }
    if col < max_col && map[row][col + 1] == c {
        neighbors.push(Point(row, col + 1));
    }
    if row > 0 && map[row - 1][col] == c {
        neighbors.push(Point(row - 1, col));
    }
    if row < max_row && map[row + 1][col] == c {
        neighbors.push(Point(row + 1, col));
    }
    neighbors
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct BadPoint(i64, i64);

impl BadPoint {
    fn left(&self) -> Self {
        BadPoint(self.0 - 1, self.1)
    }

    fn down(&self) -> Self {
        BadPoint(self.0, self.1 - 1)
    }
}

fn bad_neighbors(point: Point, c: char, map: &[Vec<char>]) -> ArrayVec<[BadPoint; 4]> {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut neighbors = array_vec!([BadPoint; 4]);

    let row = point.0;
    let col = point.1;
    if col == 0 || map[row][col - 1] != c {
        neighbors.push(BadPoint(row as i64, col as i64 - 1));
    }
    if col == max_col || map[row][col + 1] != c {
        neighbors.push(BadPoint(row as i64, col as i64 + 1));
    }
    if row == 0 || map[row - 1][col] != c {
        neighbors.push(BadPoint(row as i64 - 1, col as i64));
    }
    if row == max_row || map[row + 1][col] != c {
        neighbors.push(BadPoint(row as i64 + 1, col as i64));
    }
    neighbors
}

fn find_area_and_perim(
    p: Point,
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
) -> (u64, u64, HashSet<Point>) {
    let mut perim_nodes = HashSet::new();
    perim_nodes.insert(p);
    let c = map[p.0][p.1];
    visited[p.0][p.1] = true;
    let mut vd = VecDeque::from_iter(
        neighbors(p, c, map)
            .into_iter()
            .filter(|p2| map[p2.0][p2.1] == c)
            .inspect(|p2| {
                visited[p2.0][p2.1] = true;
            }),
    );
    let mut area = 1;
    let mut perimeter = 4 - vd.len();
    while let Some(p2) = vd.pop_front() {
        area += 1;
        let ns = neighbors(p2, c, map);
        perimeter += 4 - ns.len();
        if ns.len() < 4 {
            perim_nodes.insert(p2);
        }
        ns.into_iter().for_each(|p3| {
            if !visited[p3.0][p3.1] {
                visited[p3.0][p3.1] = true;
                vd.push_back(p3);
            }
        });
    }
    (area, perimeter as u64, perim_nodes)
}

fn find_sides(perim_nodes: HashSet<Point>, map: &[Vec<char>]) -> u64 {
    let c = {
        let p = perim_nodes
            .iter()
            .next()
            .expect("should be at least one node");
        map[p.0][p.1]
    };
    let perim_node_bad_pairs = perim_nodes
        .into_iter()
        .flat_map(|p| {
            bad_neighbors(p, c, map)
                .into_iter()
                .map(|p2| (p, p2))
                .collect_vec()
        })
        .collect::<HashSet<(Point, BadPoint)>>();
    perim_node_bad_pairs
        .iter()
        .filter(|(p, bp)| {
            !(p.0 > 0 && perim_node_bad_pairs.contains(&(p.left(), bp.left()))
                || p.1 > 0 && perim_node_bad_pairs.contains(&(p.down(), bp.down())))
        })
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut visited = map
        .iter()
        .map(|v| v.iter().map(|_| false).collect_vec())
        .collect_vec();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visited[i][j] {
                continue;
            }
            let (area, perim, _) = find_area_and_perim(Point(i, j), &map, &mut visited);
            res += area * perim;
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut visited = map
        .iter()
        .map(|v| v.iter().map(|_| false).collect_vec())
        .collect_vec();
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visited[i][j] {
                continue;
            }
            let (area, _, perim_nodes) = find_area_and_perim(Point(i, j), &map, &mut visited);
            let sides = find_sides(perim_nodes, &map);
            res += area * sides;
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
