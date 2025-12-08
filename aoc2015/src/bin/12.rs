advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let total = input
        .split(['[', ']', '{', '}', ',', ':'])
        .filter_map(|s| s.parse::<i64>().ok())
        .sum();
    Some(total)
}

fn parse_object(input: &str, index: usize) -> (i64, usize) {
    let mut index = index;
    let mut s = String::new();
    let bytes = input.as_bytes();
    let mut total = 0;
    loop {
        let c = bytes[index];
        match c {
            b',' => {
                if s == "\"red\"" {
                    // drain until end of object
                    let mut depth = 1;
                    while depth != 0 {
                        index += 1;
                        match bytes[index] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => (),
                        }
                    }
                    return (0, index);
                }
                if let Ok(x) = s.parse::<i64>() {
                    total += x;
                }
                s.clear();
            }
            b':' => {
                s.clear();
            }
            b'{' => {
                let (add_t, new_index) = parse_object(input, index + 1);
                total += add_t;
                index = new_index;
            }
            b'[' => {
                let (add_t, new_index) = parse_list(input, index + 1);
                total += add_t;
                index = new_index;
            }
            b'}' => {
                if s == "\"red\"" {
                    return (0, index);
                }
                if let Ok(x) = s.parse::<i64>() {
                    total += x;
                }
                return (total, index);
            }
            other => s.push(other as char),
        }
        index += 1;
    }
}

fn parse_list(input: &str, index: usize) -> (i64, usize) {
    let mut index = index;
    let mut s = String::new();
    let bytes = input.as_bytes();
    let mut total = 0;
    loop {
        let c = bytes[index];
        match c {
            b',' => {
                if let Ok(x) = s.parse::<i64>() {
                    total += x;
                }
                s.clear();
            }
            b'{' => {
                let (add_t, new_index) = parse_object(input, index + 1);
                total += add_t;
                index = new_index;
            }
            b'[' => {
                let (add_t, new_index) = parse_list(input, index + 1);
                total += add_t;
                index = new_index;
            }
            b']' => {
                if let Ok(x) = s.parse::<i64>() {
                    total += x;
                }
                return (total, index);
            }
            other => s.push(other as char),
        }
        index += 1;
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let (total, _) = parse_object(input, 1);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }
}
