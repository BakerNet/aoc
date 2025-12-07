use std::collections::VecDeque;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(7);

type Wire = usize;

#[derive(Debug, Clone, Copy)]
enum Op {
    Reg(Wire),
    And(Wire, Wire),
    AndOne(Wire),
    Or(Wire, Wire),
    Lshift(Wire, u16),
    Rshift(Wire, u16),
    Not(Wire),
}

impl Op {
    fn parse(input: &str) -> Self {
        let parts = input.split_whitespace().collect_vec();
        if parts.len() == 1 {
            // x
            return Op::Reg(str_to_index(parts[0]));
        }
        if parts.len() == 2 {
            // NOT x
            return Op::Not(str_to_index(parts[1]));
        }
        // x OP y
        match parts[1] {
            "AND" => {
                if parts[0] == "1" {
                    Op::AndOne(str_to_index(parts[2]))
                } else {
                    Op::And(str_to_index(parts[0]), str_to_index(parts[2]))
                }
            }
            "OR" => Op::Or(str_to_index(parts[0]), str_to_index(parts[2])),
            "LSHIFT" => Op::Lshift(
                str_to_index(parts[0]),
                parts[2].parse().expect("Lshift rvalue should be u16"),
            ),
            "RSHIFT" => Op::Rshift(
                str_to_index(parts[0]),
                parts[2].parse().expect("Rshift rvalue should be u16"),
            ),
            other => panic!("Shouldn't exist {}", other),
        }
    }

    fn handle(&self, regs: &[Option<u16>]) -> Option<u16> {
        match self {
            Op::Reg(a) => regs[*a],
            Op::And(a, b) => match (regs[*a], regs[*b]) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(x), Some(y)) => Some(x & y),
            },
            Op::AndOne(a) => regs[*a].map(|x| x & 1),
            Op::Or(a, b) => match (regs[*a], regs[*b]) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(x), Some(y)) => Some(x | y),
            },
            Op::Lshift(a, n) => regs[*a].map(|x| x << n),
            Op::Rshift(a, n) => regs[*a].map(|x| x >> n),
            Op::Not(a) => regs[*a].map(|x| !x),
        }
    }
}

#[derive(Clone, Copy)]
enum Assign {
    Literal(Wire, u16),
    Expr(Wire, Op),
}

fn str_to_index(ident: &str) -> usize {
    ident
        .bytes()
        .fold(0, |num, c| num * 26 + ((c + 1 - b'a') as usize))
        - 1
}

fn parse_input(input: &str) -> (Vec<Assign>, Vec<Option<u16>>) {
    let max_regs = str_to_index("zz");
    let regs: Vec<Option<u16>> = vec![None; max_regs];
    let circuit = input.mlines(|s| {
        let mut parts = s.split(" -> ");
        let op = parts.next().expect("Should be first part");
        let wire = parts.next().expect("Should be second part");
        let wire = str_to_index(wire.trim());
        if let Ok(x) = op.trim().parse::<u16>() {
            Assign::Literal(wire, x)
        } else {
            let expr = Op::parse(op);
            Assign::Expr(wire, expr)
        }
    });
    (circuit, regs)
}

fn run_circuit(circuit: &[Assign], regs: &mut [Option<u16>]) {
    let mut circuit = circuit
        .iter()
        .filter(|a| {
            if let Assign::Literal(wire, num) = a {
                if regs[*wire].is_none() {
                    // skip literal assignment for b in second run of part 2
                    regs[*wire] = Some(*num);
                }
                return false;
            }
            true
        })
        .copied()
        .collect::<VecDeque<Assign>>();
    while !circuit.is_empty() {
        let next = circuit.pop_front().unwrap();
        let (wire, op) = if let Assign::Expr(wire, op) = &next {
            (wire, op)
        } else {
            panic!("Should only be Exprs left")
        };
        if let Some(res) = op.handle(regs) {
            regs[*wire] = Some(res);
        } else {
            circuit.push_back(next);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (circuit, mut regs) = parse_input(input);
    run_circuit(&circuit, &mut regs);
    regs[str_to_index("a")].map(|x| x as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (circuit, mut regs) = parse_input(input);
    run_circuit(&circuit, &mut regs);
    let new_b = regs[str_to_index("a")];
    let mut regs = vec![None; regs.len()];
    regs[str_to_index("b")] = new_b;
    run_circuit(&circuit, &mut regs);
    regs[str_to_index("a")].map(|x| x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(72));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(72));
    }
}
