use itertools::Itertools;

advent_of_code::solution!(1);

fn split_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u64>().expect("Should parse"))
                .collect_tuple::<(u64, u64)>()
                .expect("Should get two numbers")
        })
        .collect::<(Vec<u64>, Vec<u64>)>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut v1, mut v2) = split_input(input);
    v1.sort();
    v2.sort();
    Some(
        v1.into_iter()
            .zip(v2)
            .map(|(x, y)| if x > y { x - y } else { y - x })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (v1, v2) = split_input(input);
    let max = v2
        .iter()
        .max()
        .copied()
        .expect("There should be a max number") as usize;
    let counts = v2.into_iter().fold(vec![0_u8; max], |mut acc, x| {
        acc[x as usize - 1] += 1;
        acc
    });
    Some(
        v1.into_iter()
            .map(|x| x * counts[x as usize - 1] as u64)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
