advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    // 48 == ascii 0
    let (files, space) = input
        .trim()
        .as_bytes()
        .iter()
        .map(|x| x - 48)
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut files, mut space), (i, num)| {
                match i % 2 {
                    0 => {
                        files.push(num);
                    }
                    _ => {
                        space.push(num);
                    }
                }
                (files, space)
            },
        );
    let mut position = 0;
    let mut fp = 0;
    let mut bp = files.len() - 1;
    let mut bp_curr = files[bp];
    let mut res = 0;
    while fp < bp {
        for _ in 0..files[fp] {
            res += fp * position;
            position += 1;
        }
        for _ in 0..space[fp] {
            res += bp * position;
            position += 1;
            bp_curr -= 1;
            if bp_curr == 0 {
                bp -= 1;
                if bp == fp {
                    break;
                }
                bp_curr = files[bp];
            }
        }
        fp += 1;
    }
    while bp_curr > 0 {
        res += bp * position;
        position += 1;
        bp_curr -= 1;
    }
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos = 0_usize;
    let (files, mut space) = input
        .trim()
        .as_bytes()
        .iter()
        .map(|x| x - 48)
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut files, mut space), (i, num)| {
                match i % 2 {
                    0 => {
                        files.push((pos, num));
                    }
                    _ => {
                        space.push((pos, num));
                    }
                }
                pos += num as usize;
                (files, space)
            },
        );
    let mut bp = files.len() - 1;
    let mut res = 0;
    loop {
        let (orig_p, l) = files[bp];
        let pos = space.iter_mut().take(bp).find(|(_, x)| *x >= l);
        if let Some((new_p, x)) = pos {
            let sum_pos = (*new_p * 2 + l as usize - 1) * l as usize / 2;
            res += sum_pos * bp;
            *new_p += l as usize;
            *x -= l;
        } else {
            let sum_pos = (orig_p * 2 + l as usize - 1) * l as usize / 2;
            res += sum_pos * bp;
        }
        if bp == 0 {
            break;
        }
        bp -= 1;
    }
    Some(res as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
