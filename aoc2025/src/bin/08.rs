use std::{
    collections::{HashMap, HashSet},
    mem,
};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(8);

fn handle_conn(
    p1: (u64, u64, u64),
    p2: (u64, u64, u64),
    circuits: &mut Vec<HashSet<(u64, u64, u64)>>,
    point_to_circuit: &mut HashMap<(u64, u64, u64), usize>,
) -> usize {
    let first_p2c = point_to_circuit.get(&p1);
    let second_p2c = point_to_circuit.get(&p2);
    let mut check_circ = 0;
    match (first_p2c, second_p2c) {
        (None, None) => {
            let circuit_id = circuits.len() - 1;
            circuits[circuit_id].insert(p1);
            circuits[circuit_id].insert(p2);
            point_to_circuit.insert(p1, circuit_id);
            point_to_circuit.insert(p2, circuit_id);
            circuits.push(HashSet::new());
        }
        (None, Some(b)) => {
            let b = *b;
            circuits[b].insert(p1);
            point_to_circuit.insert(p1, b);
            check_circ = b;
        }
        (Some(a), None) => {
            let a = *a;
            circuits[a].insert(p2);
            point_to_circuit.insert(p2, a);
            check_circ = a;
        }
        (Some(a), Some(b)) => {
            let a = a.to_owned();
            let b = b.to_owned();
            if a != b {
                // merge curcuits
                let mut circ_to_merge = HashSet::new();
                mem::swap(&mut circ_to_merge, &mut circuits[b]);
                for p in circ_to_merge.into_iter() {
                    circuits[a].insert(p);
                    point_to_circuit.insert(p, a);
                }
            }
            check_circ = a
        }
    }
    check_circ
}

fn distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    // hopefully manhattan distance works
    let x = a.0.abs_diff(b.0).pow(2);
    let y = a.1.abs_diff(b.1).pow(2);
    let z = a.2.abs_diff(b.2).pow(2);
    (x + y + z).isqrt()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.mlines(|s| {
        s.split(",")
            .map(|n| n.parse::<u64>().expect("Should get int"))
            .collect_tuple::<(u64, u64, u64)>()
            .expect("Should get 3d point")
    });
    let total_points = points.len();

    let mut circuits = Vec::<HashSet<(u64, u64, u64)>>::with_capacity(total_points);
    circuits.push(HashSet::new());
    let mut point_to_circuit = HashMap::<(u64, u64, u64), usize>::new();

    #[cfg(test)]
    let num_conns = 10;
    #[cfg(not(test))]
    let num_conns = 1000;
    let conns = points
        .into_iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| distance(*a, *b))
        .take(num_conns);
    for c in conns {
        let _ = handle_conn(c.0, c.1, &mut circuits, &mut point_to_circuit);
    }
    Some(
        circuits
            .iter()
            .map(|s| s.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input.mlines(|s| {
        s.split(",")
            .map(|n| n.parse::<u64>().expect("Should get int"))
            .collect_tuple::<(u64, u64, u64)>()
            .expect("Should get 3d point")
    });
    let total_points = points.len();

    let mut circuits = Vec::<HashSet<(u64, u64, u64)>>::with_capacity(total_points);
    circuits.push(HashSet::new());
    let mut point_to_circuit = HashMap::<(u64, u64, u64), usize>::new();

    let conns = points
        .into_iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| distance(*a, *b));
    for c in conns {
        let check_circ = handle_conn(c.0, c.1, &mut circuits, &mut point_to_circuit);
        if circuits[check_circ].len() == total_points {
            return Some(c.0 .0 * c.1 .0);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
