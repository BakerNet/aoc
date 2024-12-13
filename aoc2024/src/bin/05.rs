use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let (bp, _) = input
        .lines()
        .find_position(|s| s.trim() == "")
        .expect("Should find break point");
    let pre_rules = input
        .lines()
        .take(bp)
        .fold(vec![Vec::<u8>::new(); 100], |mut acc, s| {
            let parts = s
                .trim()
                .split("|")
                .map(|x| x.parse::<u8>().expect("Should be u8"))
                .collect_tuple::<(u8, u8)>()
                .expect("Should be able to collect tuple");
            acc[parts.0 as usize].push(parts.1);
            acc
        });
    let updates = input
        .lines()
        .skip(bp + 1)
        .map(|s| {
            s.split(",")
                .map(|x| x.parse::<u8>().expect("Should also be u8"))
                .collect_vec()
        })
        .collect_vec();
    (pre_rules, updates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pre_rules, updates) = parse_input(input);
    let res = updates
        .into_iter()
        .filter_map(|nums| {
            let mut seen = [false; 100];
            let bad = nums.iter().any(|x| {
                seen[*x as usize] = true;
                pre_rules[*x as usize].iter().any(|y| seen[*y as usize])
            });
            if bad {
                None
            } else {
                Some(nums[nums.len() / 2] as u64)
            }
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (pre_rules, updates) = parse_input(input);
    let mut bad = updates
        .into_iter()
        .filter(|nums| {
            let mut seen = [false; 100];
            nums.iter().any(|x| {
                seen[*x as usize] = true;
                pre_rules[*x as usize].iter().any(|y| seen[*y as usize])
            })
        })
        .collect_vec();
    let res = bad
        .iter_mut()
        .map(|v| {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let first = v[i];
                    let test = v[j];
                    if pre_rules[test as usize].contains(&first) {
                        v[i] = test;
                        v[j] = first;
                    }
                }
            }
            v[v.len() / 2] as u64
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
