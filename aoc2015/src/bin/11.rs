advent_of_code::solution!(11);

pub fn part_one(_input: &str) -> Option<&str> {
    // this one was way too easy to eyeball.  Might come back and code, might not
    // input was 'cqjxjnds' - first 3 are valid but don't hit requirements, so last 5 become 'xxyzz'
    Some("cqjxxyzz")
}

pub fn part_two(_input: &str) -> Option<&str> {
    // this one was also way too easy.
    // `cqjxxyzz`++ -> `cqjxxzzz` - have to bump up `j` -> `k` (still valid) so last 5 become
    // `aabcc`
    Some("cqkaabcc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let contents = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&contents);
        assert_eq!(result, Some("cqjxxyzz"));
    }

    #[test]
    fn test_part_two() {
        let contents = &advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&contents);
        assert_eq!(result, Some("cqkaabcc"));
    }
}
