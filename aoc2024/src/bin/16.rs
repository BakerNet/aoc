use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_utils::*;

advent_of_code::solution!(16);

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
    direction: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &[Vec<char>], start: Point, goal: Point) -> Option<u64> {
    let mut dist: HashMap<(Point, Dir), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let bounds = Bounds(map.len() - 1, map[0].len() - 1);

    heap.push(State {
        cost: 0,
        position: start,
        direction: Dir::Right, // dummy value
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == goal {
            return Some(cost as u64);
        }

        if let Some(found) = dist.get(&(position, direction)) {
            if *found < cost {
                continue;
            }
        }

        if let Some(next) = direction.next(position, bounds) {
            if map[next.0][next.1] != '#' {
                heap.push(State {
                    cost: cost + 1,
                    position: next,
                    direction,
                });
            }
        }

        for new_dir in vec![direction.cw(), direction.ccw()].into_iter() {
            let np = new_dir
                .next(position, bounds)
                .expect("np should always be valid due to wall");
            if map[np.0][np.1] == '#' {
                continue;
            }
            let next = State {
                cost: cost + 1000,
                position,
                direction: new_dir,
            };
            let new_cost = next.cost;

            if let Some(found) = dist.get(&(position, new_dir)) {
                if new_cost < *found {
                    heap.push(next);
                    dist.insert((position, new_dir), new_cost);
                }
            } else {
                heap.push(next);
                dist.insert((position, new_dir), new_cost);
            }
        }
    }
    None
}

#[derive(Clone, Eq, PartialEq)]
struct StateP2 {
    cost: usize,
    position: Point,
    direction: Dir,
    path: Vec<Point>,
}

impl Ord for StateP2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateP2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_part2(map: &[Vec<char>], start: Point, goal: Point) -> Option<u64> {
    let mut dist: HashMap<(Point, Dir), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let bounds = Bounds(map.len() - 1, map[0].len() - 1);

    heap.push(StateP2 {
        cost: 0,
        position: start,
        direction: Dir::Right, // dummy value
        path: vec![start],
    });

    let mut res = None::<u64>;
    let mut best: HashSet<Point> = HashSet::new();

    while let Some(StateP2 {
        cost,
        position,
        direction,
        path,
    }) = heap.pop()
    {
        if position == goal {
            if let Some(c) = res {
                if cost as u64 > c {
                    return Some(best.len() as u64);
                } else {
                    path.into_iter().for_each(|p| {
                        best.insert(p);
                    });
                    continue;
                }
            } else {
                res = Some(cost as u64);
                path.into_iter().for_each(|p| {
                    best.insert(p);
                });
                continue;
            }
        }

        if let Some(found) = dist.get(&(position, direction)) {
            if *found < cost {
                continue;
            }
        }

        if let Some(next) = direction.next(position, bounds) {
            if map[next.0][next.1] != '#' {
                let mut new_path = path.clone();
                new_path.push(next);
                heap.push(StateP2 {
                    cost: cost + 1,
                    position: next,
                    direction,
                    path: new_path,
                });
            }
        }

        for new_dir in vec![direction.cw(), direction.ccw()].into_iter() {
            let np = new_dir
                .next(position, bounds)
                .expect("np should always be valid due to wall");
            if map[np.0][np.1] == '#' {
                continue;
            }
            let next = StateP2 {
                cost: cost + 1000,
                position,
                direction: new_dir,
                path: path.clone(),
            };
            let new_cost = next.cost;

            if let Some(found) = dist.get(&(position, new_dir)) {
                if new_cost <= *found {
                    heap.push(next);
                    dist.insert((position, new_dir), new_cost);
                }
            } else {
                heap.push(next);
                dist.insert((position, new_dir), new_cost);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.c_map();
    let start = find_point(&map, 'S');
    let end = find_point(&map, 'E');
    dijkstra(&map, start, end)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.c_map();
    let start = find_point(&map, 'S');
    let end = find_point(&map, 'E');
    dijkstra_part2(&map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
