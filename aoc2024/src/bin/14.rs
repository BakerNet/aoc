use aoc_utils::*;
use regex::Regex;

advent_of_code::solution!(14);

#[cfg(not(test))]
const WIDTH: i64 = 101;
#[cfg(test)]
const WIDTH: i64 = 11;
#[cfg(not(test))]
const HEIGHT: i64 = 103;
#[cfg(test)]
const HEIGHT: i64 = 7;

#[derive(Clone, Copy, Debug)]
struct Bot {
    pos: (i64, i64),
    vel: (i64, i64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"^p=(\d{1,3}),(\d{1,3}) v=(-?\d{1,3}),(-?\d{1,3})$")
        .expect("Regex should be valid");
    let bots = input.regex_mlines(re, |c| {
        let px = c.get_num::<i64>(1);
        let py = c.get_num::<i64>(2);
        let vx = c.get_num::<i64>(3);
        let vy = c.get_num::<i64>(4);
        Bot {
            pos: (px, py),
            vel: (vx, vy),
        }
    });
    let mut quads = [0; 4];
    bots.into_iter().for_each(|b| {
        let mut fx = (b.pos.0 + (b.vel.0 * 100)) % WIDTH;
        if fx < 0 {
            fx += WIDTH;
        }
        let mut fy = (b.pos.1 + (b.vel.1 * 100)) % HEIGHT;
        if fy < 0 {
            fy += HEIGHT;
        }
        match (fx, fy) {
            (x, y) if x < WIDTH / 2 && y < HEIGHT / 2 => {
                quads[0] += 1;
            }
            (x, y) if x > WIDTH / 2 && y < HEIGHT / 2 => {
                quads[1] += 1;
            }
            (x, y) if x < WIDTH / 2 && y > HEIGHT / 2 => {
                quads[2] += 1;
            }
            (x, y) if x > WIDTH / 2 && y > HEIGHT / 2 => {
                quads[3] += 1;
            }
            (_, _) => {}
        }
    });
    Some(quads[0] * quads[1] * quads[2] * quads[3])
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"^p=(\d{1,3}),(\d{1,3}) v=(-?\d{1,3}),(-?\d{1,3})$")
        .expect("Regex should be valid");
    let bots = input.regex_mlines(re, |c| {
        let px = c.get_num::<i64>(1);
        let py = c.get_num::<i64>(2);
        let vx = c.get_num::<i64>(3);
        let vy = c.get_num::<i64>(4);
        Bot {
            pos: (px, py),
            vel: (vx, vy),
        }
    });

    // pattern should repeat every HEIGHT * WIDTH steps... but when does it start?
    for i in 0..103 {
        let mut map = vec![vec!['.'; 101]; 103];
        bots.iter().for_each(|b| {
            let mut fx = (b.pos.0 + (b.vel.0 * i)) % WIDTH;
            if fx < 0 {
                fx += WIDTH;
            }
            let mut fy = (b.pos.1 + (b.vel.1 * i)) % HEIGHT;
            if fy < 0 {
                fy += HEIGHT;
            }
            map[fy as usize][fx as usize] = '0'
        });
        // println!("\n\n----- {} ------", i);
        #[allow(unused_variables)]
        map.iter().for_each(|v| {
            // v.iter().for_each(|c| print!("{c}"));
            // print!("\n");
        });
    }

    // manually looking at output, see some sort of pattern at 19 and a different one at 70
    //   pattern @ 19: Horizontal "ribbon"
    //   pattern @ 70: Vertical "ribbon"
    //   When the horizontal & vertical "ribbons" line up is where the tree will be.
    //
    // pattern at 19 repeats every 103 (height)
    //  x*103 + 19 = res
    // pattern at 70 repeats every 101 (width)
    //  y*101 + 70 = res
    //
    // y = (103x - 51)/101

    // NOTE: These constants (19 & 70) will differ for different input - use the prints to find
    // where your patterns occur
    const VERTICAL_PATTERN: f64 = 19.0;
    const HORIZONTAL_PATTERN: f64 = 70.0;

    let mut x = 0;
    for i in 1..101 {
        let res =
            (HEIGHT as f64 * (i as f64) - (HORIZONTAL_PATTERN - VERTICAL_PATTERN)) / WIDTH as f64;
        // find first whole number - this is our x in x * 103 + 19 = res
        if res.fract() == 0.0 {
            x = i;
            break;
        }
    }
    let x = 103 * x + 19;
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // doesn't actually make a tree in example input
        assert_eq!(result, Some(122));
    }
}
