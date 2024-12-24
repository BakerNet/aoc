use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct RuleParts {
    left: String,
    right: String,
    out: String,
}

#[derive(Debug, Clone)]
enum Rule {
    And(RuleParts),
    Or(RuleParts),
    Xor(RuleParts),
}

impl Rule {
    fn from_strs(left: &str, rule: &str, right: &str, out: &str) -> Self {
        match rule {
            "AND" => Rule::And(RuleParts {
                left: left.to_owned(),
                right: right.to_owned(),
                out: out.to_owned(),
            }),
            "OR" => Rule::Or(RuleParts {
                left: left.to_owned(),
                right: right.to_owned(),
                out: out.to_owned(),
            }),
            "XOR" => Rule::Xor(RuleParts {
                left: left.to_owned(),
                right: right.to_owned(),
                out: out.to_owned(),
            }),
            _ => panic!("Unknown rule: {}", rule),
        }
    }

    fn left(&self) -> &str {
        match self {
            Rule::And(parts) => &parts.left,
            Rule::Or(parts) => &parts.left,
            Rule::Xor(parts) => &parts.left,
        }
    }

    fn right(&self) -> &str {
        match self {
            Rule::And(parts) => &parts.right,
            Rule::Or(parts) => &parts.right,
            Rule::Xor(parts) => &parts.right,
        }
    }

    fn out(&self) -> &str {
        match self {
            Rule::And(parts) => &parts.out,
            Rule::Or(parts) => &parts.out,
            Rule::Xor(parts) => &parts.out,
        }
    }

    fn apply(&self, registers: &mut HashMap<String, u64>) {
        let left = registers.get(self.left()).unwrap();
        let right = registers.get(self.right()).unwrap();
        let out = match self {
            Rule::And(_) => left & right,
            Rule::Or(_) => left | right,
            Rule::Xor(_) => left ^ right,
        };
        registers.insert(self.out().to_owned(), out);
    }
}

fn parse_input(input: &str) -> (HashMap<String, u64>, Vec<Rule>) {
    let blocks = input.blocks();
    let registers = blocks[0]
        .mlines(|s| {
            let parts = s.split_once(": ").expect("Should have a colon");

            (
                parts.0.to_owned(),
                parts.1.parse::<u64>().expect("Should be a number"),
            )
        })
        .into_iter()
        .collect::<HashMap<String, u64>>();
    let rules = blocks[1].mlines(|l| {
        let mut parts = l.split_whitespace();
        Rule::from_strs(
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.nth(1).unwrap(),
        )
    });
    (registers, rules)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut registers, rules) = parse_input(input);
    let mut queue = VecDeque::from(rules);
    while !queue.is_empty() {
        let rule = queue.pop_front().unwrap();
        if !registers.contains_key(rule.left()) || !registers.contains_key(rule.right()) {
            queue.push_back(rule);
            continue;
        }
        rule.apply(&mut registers);
    }
    let mut zs = registers
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect::<Vec<_>>();
    zs.sort();
    Some(zs.into_iter().rev().fold(0, |acc, (_, v)| acc << 1 | v))
}

pub fn part_two(input: &str) -> Option<String> {
    let (registers, rules) = parse_input(input);
    let max_number = registers.len() / 2;

    // FULL ADDER
    // (first bits aren't a full adder)
    // (for last FA, COUT is the extra output)
    //
    // Xn    XOR  Yn    -> VAL0  <= FAGate0
    // Xn    AND  Yn    -> VAL1  <= FAGate1
    // VAL0  AND  CIN   -> VAL2  <= FAGate2
    // VAL0  XOR  CIN   -> Zn    <= FAGate3
    // VAL1  OR   VAL2  -> COUT  <= FAGate4

    let mut bad_outputs = HashSet::new();
    // check FAGate0 gates for Zns
    // each of these should be Xn XOR Yn -> VAL0n
    // except for the first one, which should be x00 XOR y00 -> z00
    let fa0s = rules
        .iter()
        .filter(|r| {
            if let Rule::Xor(parts) = r {
                (parts.left.starts_with("x") || parts.right.starts_with("x"))
                    && (parts.left.starts_with("y") || parts.right.starts_with("y"))
            } else {
                false
            }
        })
        .collect_vec();
    fa0s.iter().for_each(|r| {
        if r.out().starts_with("z") && r.out() != "z00" && (r.left() != "x00" || r.right() != "x00")
        {
            bad_outputs.insert(r.out());
        }
    });

    // check all XOR gates that do not take Xn or Yn inputs (FAGate3)
    // each of these should be outputting to a zXX
    let fa3s = rules
        .iter()
        .filter(|r| {
            if let Rule::Xor(parts) = r {
                !(parts.left.starts_with("x")
                    || parts.right.starts_with("x")
                    || parts.left.starts_with("y")
                    || parts.right.starts_with("y"))
            } else {
                false
            }
        })
        .collect_vec();
    fa3s.iter().for_each(|r| {
        if !r.out().starts_with("z") {
            bad_outputs.insert(r.out());
        }
    });

    // check all output gates
    // each of these should be VAL0 XOR CIN -> Zn (FAGate3)
    // except for the last one, which should be VAL1 OR VAL2 -> COUT
    let outputs = rules
        .iter()
        .filter(|r| r.out().starts_with("z"))
        .collect_vec();
    let max_output = format!("z{:02}", max_number);
    outputs.iter().for_each(|r| {
        if r.out() == max_output {
            if !matches!(r, Rule::Or(_)) {
                bad_outputs.insert(r.out());
            }
        } else {
            if !matches!(r, Rule::Xor(_)) {
                bad_outputs.insert(r.out());
            }
        }
    });

    // all FAGate0 gates MUST be inputs to a FAGate3 gate
    let bad_fa0s = fa0s
        .iter()
        .filter(|r| {
            if r.out() == "z00" {
                return false;
            }
            !fa3s
                .iter()
                .any(|r2| r2.left() == r.out() || r2.right() == r.out())
        })
        .collect_vec();
    bad_fa0s.iter().for_each(|r| {
        bad_outputs.insert(r.out());
        let intended_out = format!("z{}", &r.left()[1..]);
        let expected_fa3 = fa3s
            .iter()
            .find(|r2| r2.out() == intended_out)
            .expect("Shoud have a next");
        let left = expected_fa3.left();
        let right = expected_fa3.right();
        // find an FAGate4 that outputs a CIN used as input to the expected FAGate3
        // the other input should be the output of the bad FAGate0
        rules.iter().for_each(|r3| {
            if matches!(r3, Rule::Or(_)) {
                if r3.out() == left {
                    bad_outputs.insert(right);
                }
                if r3.out() == right {
                    bad_outputs.insert(left);
                }
            }
        });
    });
    Some(bad_outputs.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2024));
    }
}
