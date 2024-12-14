use itertools::Itertools;
use regex::{Captures, Regex};

pub trait InputParse<'a> {
    fn mlines<F, U>(self, f: F) -> Vec<U>
    where
        F: Fn(&str) -> U + 'static + Copy;

    fn regex_mlines<F, U>(self, re: Regex, f: F) -> Vec<U>
    where
        F: Fn(Captures) -> U + 'static + Copy;

    fn c_map(self) -> Vec<Vec<char>>;

    fn c_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(char) -> U + 'static + Copy;

    fn ws_map(self) -> Vec<Vec<&'a str>>;

    fn ws_mmap<F, U>(self, f: F) -> Vec<Vec<U>>
    where
        F: Fn(&str) -> U + 'static + Copy;

    fn blocks(self) -> Vec<&'a str>;

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
