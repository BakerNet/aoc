use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<String, usize>) {
    let mut machine_ids = HashMap::new();
    let s_to_id = move |m: String, machine_ids: &mut HashMap<String, usize>| {
        if let Some(x) = machine_ids.get(&m) {
            *x
        } else {
            let curr = machine_ids.len();
            machine_ids.insert(m, curr);
            curr
        }
    };
    let mut machines = HashMap::new();
    input.lines().for_each(|s| {
        let mut parts = s.split_whitespace();
        let machine = parts
            .next()
            .expect("Should find machine")
            .trim_end_matches(':')
            .to_string();
        let m = s_to_id(machine, &mut machine_ids);
        let outputs = parts
            .map(|x| s_to_id(x.to_string(), &mut machine_ids))
            .collect_vec();
        machines.insert(m, outputs);
    });
    (machines, machine_ids)
}

fn recurse_paths(
    m: usize,
    end: usize,
    machines: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    if m == end {
        return 1;
    }
    if let Some(x) = memo.get(&m) {
        return *x;
    }
    let mut total = 0;
    for x in machines.get(&m).expect("All outputs should be inputs") {
        total += recurse_paths(*x, end, machines, memo);
    }
    memo.insert(m, total);
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let (machines, machine_ids) = parse_input(input);
    let start = machine_ids.get("you").unwrap();
    let end = machine_ids.get("out").unwrap();
    let mut seen = HashMap::new();
    let total = recurse_paths(*start, *end, &machines, &mut seen);
    Some(total)
}

fn recurse_paths_part2(
    m: usize,
    end: usize,
    dac: usize,
    fft: usize,
    mut seen_dac: bool,
    mut seen_fft: bool,
    machines: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<(usize, bool, bool), u64>,
) -> u64 {
    if m == end {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }
    if m == dac {
        seen_dac = true;
    }
    if m == fft {
        seen_fft = true;
    }
    if let Some(x) = memo.get(&(m, seen_dac, seen_fft)) {
        return *x;
    }

    let mut total = 0;
    for x in machines.get(&m).expect("All outputs should be inputs") {
        total += recurse_paths_part2(*x, end, dac, fft, seen_dac, seen_fft, machines, memo);
    }
    memo.insert((m, seen_dac, seen_fft), total);
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let (machines, machine_ids) = parse_input(input);
    let start = machine_ids.get("svr").unwrap();
    let end = machine_ids.get("out").unwrap();
    let dac = machine_ids.get("dac").unwrap();
    let fft = machine_ids.get("fft").unwrap();
    let mut seen = HashMap::new();
    let total = recurse_paths_part2(*start, *end, *dac, *fft, false, false, &machines, &mut seen);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
