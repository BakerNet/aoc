use std::collections::HashMap;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let nice = input
        .mlines(move |s| {
            let mut vowel_count =
                if vowels.contains(&s.chars().next().expect("Should be a first leter")) {
                    1
                } else {
                    0
                };
            let mut doubles = false;
            let bad = s.chars().tuple_windows().any(|(a, b)| {
                if vowel_count < 3 && vowels.contains(&b) {
                    vowel_count += 1;
                }
                if a == b {
                    doubles = true;
                }
                matches!((a, b), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y'))
            });
            vowel_count >= 3 && doubles && !bad
        })
        .into_iter()
        .filter(|b| *b)
        .count();
    Some(nice as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nice = input
        .mlines(move |s| {
            let mut pairs = HashMap::<(char, char), usize>::new();
            let first_two = s
                .chars()
                .take(2)
                .collect_tuple::<(char, char)>()
                .expect("Should be at least 2 chars");
            pairs.insert(first_two, 0);
            let mut repeated_pair = false;
            let mut doubles = false;
            s.chars().tuple_windows().enumerate().any(|(i, (a, b, c))| {
                let pair_index = i + 1;
                pairs
                    .entry((b, c))
                    .and_modify(|t| {
                        if pair_index > *t + 1 {
                            repeated_pair = true;
                        }
                    })
                    .or_insert(pair_index);
                if a == c {
                    doubles = true;
                }
                repeated_pair && doubles
            })
        })
        .into_iter()
        .filter(|b| *b)
        .count();
    Some(nice as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
