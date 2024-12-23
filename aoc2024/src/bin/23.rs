use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut cons: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let comps = l.split_once("-").expect("Split should work");
        cons.entry(comps.0)
            .and_modify(|v| v.push(comps.1))
            .or_insert(vec![comps.1]);
        cons.entry(comps.1)
            .and_modify(|v| v.push(comps.0))
            .or_insert(vec![comps.0]);
    });
    cons
}

pub fn part_one(input: &str) -> Option<u64> {
    let cons = parse_input(input);
    let mut threes = HashSet::new();
    cons.iter()
        .filter(|x| x.0.starts_with("t"))
        .for_each(|(&k, v)| {
            v.iter().for_each(|&k2| {
                cons.get(k2)
                    .unwrap()
                    .iter()
                    .filter(|&&k3| k3 != k)
                    .for_each(|&k3| {
                        if cons.get(k3).unwrap().contains(&k) {
                            let mut new = [k, k2, k3];
                            new.sort();
                            threes.insert(new);
                        }
                    })
            })
        });
    Some(threes.len() as u64)
}

fn largest_containing<'a>(
    containing: Vec<&'a str>,
    tried_containing: &mut HashSet<String>,
    cons: &'a HashMap<&'a str, Vec<&'a str>>,
) -> String {
    let item = containing.iter().next().unwrap();
    let v = cons.get(item).unwrap();
    let res = v
        .iter()
        .filter_map(|&k| {
            if containing.contains(&k) {
                return None;
            }
            let other_v = cons.get(k).unwrap();
            for j in containing.iter() {
                if !other_v.contains(j) {
                    return None;
                }
            }
            let mut new_containing = containing.clone();
            new_containing.push(k);
            new_containing.sort();
            let as_string = new_containing.join(",");
            if tried_containing.contains(&as_string) {
                return None;
            }
            tried_containing.insert(as_string);
            Some(largest_containing(new_containing, tried_containing, cons))
        })
        .fold(
            String::new(),
            |acc, s| {
                if s.len() > acc.len() {
                    s
                } else {
                    acc
                }
            },
        );
    if res.is_empty() {
        containing.join(",")
    } else {
        res
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let cons = parse_input(input);
    let mut seen_sets = HashSet::new();
    let largest = cons
        .keys()
        .map(|&k| largest_containing(vec![k], &mut seen_sets, &cons))
        .fold(
            String::new(),
            |acc, s| {
                if s.len() > acc.len() {
                    s
                } else {
                    acc
                }
            },
        );
    Some(largest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
