use std::{fmt::Debug, str::FromStr};

use itertools::Itertools;
use regex::{Captures, Regex};
use tinyvec::{ArrayVec, array_vec};

/// 2D grid point: `Point(row, col)`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct Point(pub usize, pub usize);

/// Find first occurrence of value in 2D grid
pub fn find_point<U>(map: &[Vec<U>], val: U) -> Point
where
    U: PartialEq,
{
    map.iter()
        .enumerate()
        .find_map(|(row, v)| {
            v.iter().enumerate().find_map(|(col, item)| {
                if *item == val {
                    Some(Point(row, col))
                } else {
                    None
                }
            })
        })
        .expect("Should find point")
}

/// Grid boundaries: `Bounds(max_row, max_col)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds(pub usize, pub usize);

/// 2D grid point with signed coords: `IPoint(row, col)`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct IPoint(pub i64, pub i64);

/// 4-directional navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// Manhattan distance between two points
pub fn dist(p: Point, p2: Point) -> usize {
    let x = p.0.abs_diff(p2.0);
    let y = p.1.abs_diff(p2.1);
    x + y
}

impl Dir {
    /// Get all 4-directional neighbors within bounds
    pub fn neighbors(p: Point, b: Bounds) -> ArrayVec<[Point; 4]> {
        let mut ns = array_vec!([Point; 4]);

        if p.0 > 0 {
            ns.push(Point(p.0 - 1, p.1));
        }
        if p.0 < b.0 {
            ns.push(Point(p.0 + 1, p.1));
        }
        if p.1 > 0 {
            ns.push(Point(p.0, p.1 - 1));
        }
        if p.1 < b.1 {
            ns.push(Point(p.0, p.1 + 1));
        }
        ns
    }

    /// Get all 4-directional neighbors (unbounded, signed)
    pub fn ineighbors(p: IPoint) -> ArrayVec<[IPoint; 4]> {
        let mut ns = array_vec!([IPoint; 4]);

        ns.push(IPoint(p.0 - 1, p.1));
        ns.push(IPoint(p.0 + 1, p.1));
        ns.push(IPoint(p.0, p.1 - 1));
        ns.push(IPoint(p.0, p.1 + 1));
        ns
    }

    /// Rotate clockwise
    pub fn cw(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    /// Rotate counter-clockwise
    pub fn ccw(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    /// Move one step in this direction (bounded, returns None if out of bounds)
    pub fn next(&self, p: Point, b: Bounds) -> Option<Point> {
        match self {
            Dir::Up => {
                if p.0 > 0 {
                    Some(Point(p.0 - 1, p.1))
                } else {
                    None
                }
            }
            Dir::Down => {
                if p.0 < b.0 {
                    Some(Point(p.0 + 1, p.1))
                } else {
                    None
                }
            }
            Dir::Left => {
                if p.1 > 0 {
                    Some(Point(p.0, p.1 - 1))
                } else {
                    None
                }
            }
            Dir::Right => {
                if p.1 < b.1 {
                    Some(Point(p.0, p.1 + 1))
                } else {
                    None
                }
            }
        }
    }

    /// Move one step in this direction (unbounded, may underflow)
    pub fn inext(&self, p: Point) -> Point {
        match self {
            Dir::Up => Point(p.0 - 1, p.1),
            Dir::Down => Point(p.0 + 1, p.1),
            Dir::Left => Point(p.0, p.1 - 1),
            Dir::Right => Point(p.0, p.1 + 1),
        }
    }
}

/// 8-directional navigation (includes diagonals)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirExt {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl DirExt {
    /// Get all 8-directional neighbors within bounds
    pub fn neighbors(p: Point, b: Bounds) -> ArrayVec<[Point; 8]> {
        let mut ns = array_vec!([Point; 8]);

        if p.0 > 0 {
            ns.push(Point(p.0 - 1, p.1));

            if p.1 > 0 {
                ns.push(Point(p.0 - 1, p.1 - 1));
            }
            if p.1 < b.1 {
                ns.push(Point(p.0 - 1, p.1 + 1));
            }
        }
        if p.0 < b.0 {
            ns.push(Point(p.0 + 1, p.1));

            if p.1 > 0 {
                ns.push(Point(p.0 + 1, p.1 - 1));
            }
            if p.1 < b.1 {
                ns.push(Point(p.0 + 1, p.1 + 1));
            }
        }
        if p.1 > 0 {
            ns.push(Point(p.0, p.1 - 1));
        }
        if p.1 < b.1 {
            ns.push(Point(p.0, p.1 + 1));
        }
        ns
    }

    /// Get all 8-directional neighbors (unbounded, signed)
    pub fn ineighbors(p: IPoint) -> ArrayVec<[IPoint; 8]> {
        let mut ns = array_vec!([IPoint; 8]);

        ns.push(IPoint(p.0 - 1, p.1));
        ns.push(IPoint(p.0 - 1, p.1 - 1));
        ns.push(IPoint(p.0 - 1, p.1 + 1));
        ns.push(IPoint(p.0 + 1, p.1));
        ns.push(IPoint(p.0 + 1, p.1 - 1));
        ns.push(IPoint(p.0 + 1, p.1 + 1));
        ns.push(IPoint(p.0, p.1 - 1));
        ns.push(IPoint(p.0, p.1 + 1));
        ns
    }

    /// Move one step in this direction (bounded, returns None if out of bounds)
    pub fn next(&self, p: Point, b: Bounds) -> Option<Point> {
        match self {
            DirExt::Up => {
                if p.0 > 0 {
                    Some(Point(p.0 - 1, p.1))
                } else {
                    None
                }
            }
            DirExt::UpLeft => {
                if p.0 > 0 && p.1 > 0 {
                    Some(Point(p.0 - 1, p.1 - 1))
                } else {
                    None
                }
            }
            DirExt::UpRight => {
                if p.0 > 0 && p.1 < b.1 {
                    Some(Point(p.0 - 1, p.1 + 1))
                } else {
                    None
                }
            }
            DirExt::Down => {
                if p.0 < b.0 {
                    Some(Point(p.0 + 1, p.1))
                } else {
                    None
                }
            }
            DirExt::DownLeft => {
                if p.0 < b.0 && p.1 > 0 {
                    Some(Point(p.0 + 1, p.1 - 1))
                } else {
                    None
                }
            }
            DirExt::DownRight => {
                if p.0 < b.0 && p.1 < b.1 {
                    Some(Point(p.0 + 1, p.1 + 1))
                } else {
                    None
                }
            }
            DirExt::Left => {
                if p.1 > 0 {
                    Some(Point(p.0, p.1 - 1))
                } else {
                    None
                }
            }
            DirExt::Right => {
                if p.1 < b.1 {
                    Some(Point(p.0, p.1 + 1))
                } else {
                    None
                }
            }
        }
    }

    /// Move one step in this direction (unbounded, may underflow)
    pub fn inext(&self, p: Point) -> Point {
        match self {
            DirExt::Up => Point(p.0 - 1, p.1),
            DirExt::UpLeft => Point(p.0 - 1, p.1 - 1),
            DirExt::UpRight => Point(p.0 - 1, p.1 + 1),
            DirExt::Down => Point(p.0 + 1, p.1),
            DirExt::DownLeft => Point(p.0 + 1, p.1 - 1),
            DirExt::DownRight => Point(p.0 + 1, p.1 + 1),
            DirExt::Left => Point(p.0, p.1 - 1),
            DirExt::Right => Point(p.0, p.1 + 1),
        }
    }
}

/// Add signed int to usize (handles negative)
pub fn iadd<I>(x: usize, y: I) -> usize
where
    I: Into<i64>,
{
    let y = y.into();
    if y < 0 {
        x - (-y) as usize
    } else {
        x + y as usize
    }
}

/// Add signed int to u64 (handles negative)
pub fn iadd64<I>(x: u64, y: I) -> u64
where
    I: Into<i64>,
{
    let y = y.into();
    if y < 0 { x - (-y) as u64 } else { x + y as u64 }
}

/// Parse str to u64
pub fn ufroms(s: &str) -> u64 {
    s.parse().expect("Should parse number")
}

/// Parse char digit to u32
pub fn ufromc(c: char) -> u32 {
    c.to_digit(10).expect("Should parse number")
}

/// Input parsing convenience methods
pub trait InputParse<'a> {
    /// Map function over each line
    fn mlines<F, U>(self, f: F) -> Vec<U>
    where
        F: Fn(&str) -> U + 'static + Copy;

    /// Apply regex to each line and map captures
    fn regex_mlines<F, U>(self, re: Regex, f: F) -> Vec<U>
    where
        F: Fn(Captures) -> U + 'static + Copy;

    /// Parse to char grid `Vec<Vec<char>>`
    fn c_map(self) -> Vec<Vec<char>>;

    /// Parse to char grid with mapping function
    fn c_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(char) -> U + 'static + Copy;

    /// Split lines by whitespace to `Vec<Vec<&str>>`
    fn ws_map(self) -> Vec<Vec<&'a str>>;

    /// Split lines by whitespace and map each token
    fn ws_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(&str) -> U + 'static + Copy;

    /// Split input by double newlines
    fn blocks(self) -> Vec<&'a str>;

    /// Split by double newlines and map each block
    fn mblocks<F, U>(self, f: F) -> Vec<U>
    where
        F: Fn(&str) -> U + 'static + Copy;
}

impl<'a> InputParse<'a> for &'a str {
    fn mlines<F, U>(self, f: F) -> Vec<U>
    where
        F: Fn(&str) -> U + 'static + Copy,
    {
        self.lines().map(f).collect_vec()
    }

    fn regex_mlines<F, U>(self, re: Regex, f: F) -> Vec<U>
    where
        F: Fn(Captures) -> U + 'static + Copy,
    {
        self.lines()
            .map(|l| re.captures(l).map(f).expect("Regex should work"))
            .collect_vec()
    }

    fn c_map(self) -> Vec<Vec<char>> {
        self.lines().map(|l| l.chars().collect_vec()).collect_vec()
    }

    fn c_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(char) -> U + 'static + Copy,
    {
        self.lines()
            .map(|l| l.chars().map(f).collect_vec())
            .collect_vec()
    }

    fn ws_map(self) -> Vec<Vec<&'a str>> {
        self.lines()
            .map(|l| l.split_whitespace().collect_vec())
            .collect_vec()
    }

    fn ws_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(&str) -> U + 'static + Copy,
    {
        self.lines()
            .map(|l| l.split_whitespace().map(f).collect_vec())
            .collect_vec()
    }

    fn blocks(self) -> Vec<&'a str> {
        self.split("\n\n").collect_vec()
    }

    fn mblocks<F, U>(self, f: F) -> Vec<U>
    where
        F: Fn(&str) -> U + 'static + Copy,
    {
        self.split("\n\n").map(f).collect_vec()
    }
}

/// Extract numbers from regex captures
pub trait ExtractNum {
    /// Parse capture group at position to number type
    fn get_num<U>(&self, pos: usize) -> U
    where
        U: FromStr,
        <U as FromStr>::Err: Debug;
}

impl ExtractNum for Captures<'_> {
    fn get_num<U>(&self, pos: usize) -> U
    where
        U: FromStr,
        <U as FromStr>::Err: Debug,
    {
        self.get(pos)
            .expect("Should get px")
            .as_str()
            .parse::<U>()
            .expect("Capture should be int")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iadd_works() {
        assert_eq!(iadd(12, -6), 6);
        assert_eq!(iadd(12, 6), 18);
        assert_eq!(iadd64(12, -6), 6);
        assert_eq!(iadd64(12, 6), 18);
    }

    #[test]
    fn test_dir_neighbors() {
        let bounds = Bounds(4, 4);
        let point = Point(2, 4);

        let neighbors = Dir::neighbors(point, bounds);
        let expected = vec![
            Point(1, 4), // Up
            Point(3, 4), // Down
            Point(2, 3), // Left
                         // Right excluded because on edge
        ];

        assert_eq!(neighbors.into_iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_dir_ineighbors() {
        let point = IPoint(0, 2);

        let neighbors = Dir::ineighbors(point);
        let expected = vec![
            IPoint(-1, 2), // Up
            IPoint(1, 2),  // Down
            IPoint(0, 1),  // Left
            IPoint(0, 3),  // Right
        ];

        assert_eq!(neighbors.into_iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_dir_cw() {
        assert_eq!(Dir::Up.cw(), Dir::Right);
        assert_eq!(Dir::Right.cw(), Dir::Down);
        assert_eq!(Dir::Down.cw(), Dir::Left);
        assert_eq!(Dir::Left.cw(), Dir::Up);
    }

    #[test]
    fn test_dir_ccw() {
        assert_eq!(Dir::Up.ccw(), Dir::Left);
        assert_eq!(Dir::Left.ccw(), Dir::Down);
        assert_eq!(Dir::Down.ccw(), Dir::Right);
        assert_eq!(Dir::Right.ccw(), Dir::Up);
    }

    #[test]
    fn test_dir_next() {
        let bounds = Bounds(4, 4);
        let point = Point(2, 2);

        assert_eq!(Dir::Up.next(point, bounds), Some(Point(1, 2)));
        assert_eq!(Dir::Down.next(point, bounds), Some(Point(3, 2)));
        assert_eq!(Dir::Left.next(point, bounds), Some(Point(2, 1)));
        assert_eq!(Dir::Right.next(point, bounds), Some(Point(2, 3)));

        // Out of bounds
        assert_eq!(Dir::Up.next(Point(0, 2), bounds), None);
        assert_eq!(Dir::Left.next(Point(2, 0), bounds), None);
        assert_eq!(Dir::Down.next(Point(4, 2), bounds), None);
        assert_eq!(Dir::Right.next(Point(2, 4), bounds), None);
    }

    #[test]
    fn test_dir_inext() {
        let point = Point(2, 2);

        assert_eq!(Dir::Up.inext(point), Point(1, 2));
        assert_eq!(Dir::Down.inext(point), Point(3, 2));
        assert_eq!(Dir::Left.inext(point), Point(2, 1));
        assert_eq!(Dir::Right.inext(point), Point(2, 3));
    }

    #[test]
    fn test_dir_ext_neighbors() {
        let bounds = Bounds(4, 4);
        let point = Point(2, 4);

        let neighbors = DirExt::neighbors(point, bounds);
        let expected = vec![
            Point(1, 4), // Up
            Point(1, 3), // UpLeft
            // UpRight excluded
            Point(3, 4), // Down
            Point(3, 3), // DownLeft
            // DownRight excluded
            Point(2, 3), // Left
                         // Right excluded
        ];

        assert_eq!(neighbors.into_iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_dir_ext_ineighbors() {
        let point = IPoint(0, 2);

        let neighbors = DirExt::ineighbors(point);
        let expected = vec![
            IPoint(-1, 2), // Up
            IPoint(-1, 1), // UpLeft
            IPoint(-1, 3), // UpRight
            IPoint(1, 2),  // Down
            IPoint(1, 1),  // DownLeft
            IPoint(1, 3),  // DownRight
            IPoint(0, 1),  // Left
            IPoint(0, 3),  // Right
        ];

        assert_eq!(neighbors.into_iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_dir_ext_next() {
        let bounds = Bounds(4, 4);
        let point = Point(2, 2);

        assert_eq!(DirExt::Up.next(point, bounds), Some(Point(1, 2)));
        assert_eq!(DirExt::UpLeft.next(point, bounds), Some(Point(1, 1)));
        assert_eq!(DirExt::UpRight.next(point, bounds), Some(Point(1, 3)));
        assert_eq!(DirExt::Down.next(point, bounds), Some(Point(3, 2)));
        assert_eq!(DirExt::DownLeft.next(point, bounds), Some(Point(3, 1)));
        assert_eq!(DirExt::DownRight.next(point, bounds), Some(Point(3, 3)));
        assert_eq!(DirExt::Left.next(point, bounds), Some(Point(2, 1)));
        assert_eq!(DirExt::Right.next(point, bounds), Some(Point(2, 3)));

        // Out of bounds
        assert_eq!(DirExt::Up.next(Point(0, 2), bounds), None);
        assert_eq!(DirExt::UpLeft.next(Point(0, 0), bounds), None);
    }

    #[test]
    fn test_dir_ext_inext() {
        let point = Point(2, 2);

        assert_eq!(DirExt::Up.inext(point), Point(1, 2));
        assert_eq!(DirExt::UpLeft.inext(point), Point(1, 1));
        assert_eq!(DirExt::UpRight.inext(point), Point(1, 3));
        assert_eq!(DirExt::Down.inext(point), Point(3, 2));
        assert_eq!(DirExt::DownLeft.inext(point), Point(3, 1));
        assert_eq!(DirExt::DownRight.inext(point), Point(3, 3));
        assert_eq!(DirExt::Left.inext(point), Point(2, 1));
        assert_eq!(DirExt::Right.inext(point), Point(2, 3));
    }
}
