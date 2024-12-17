use std::ops::BitXor;

use aoc_utils::*;
use itertools::Itertools;

advent_of_code::solution!(17);

struct Cpu {
    program: Vec<Op>,
    pointer: usize,
    output: Vec<u64>,
    registers: [u64; 3],
}

impl Cpu {
    fn run_program(mut self) -> Vec<u64> {
        while self.pointer < self.program.len() {
            let op = self.program[self.pointer];
            self.pointer += op.run(&mut self);
        }
        self.output
    }
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
    C,
}

impl Reg {
    fn index(&self) -> usize {
        match self {
            Reg::A => 0,
            Reg::B => 1,
            Reg::C => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Adv(COp),   // 0 - divide A by COp ^ 2 > A
    Bxl(LitOp), // 1 - bitwise XOR on B by LitOp > B
    Bst(COp),   // 2 - COp % 8 > B
    Jnz(LitOp), // 3 - nothing if A == 0, else jumpt pointer to A
    Bxc,        // 4 - bitwise XOR of B & C > B
    Out(COp),   // 5 - COp % 8 > output
    Bdv(COp),   // 6 - divide A by COp ^ 2 > B
    Cdv(COp),   // 7 - divide A by COp ^ 2 > C
}

impl Op {
    fn from_chars(opcode: char, operand: char) -> Self {
        match opcode {
            '0' => Op::Adv(COp::from_char(operand)),
            '1' => Op::Bxl(LitOp::from_char(operand)),
            '2' => Op::Bst(COp::from_char(operand)),
            '3' => Op::Jnz(LitOp::from_char(operand)),
            '4' => Op::Bxc,
            '5' => Op::Out(COp::from_char(operand)),
            '6' => Op::Bdv(COp::from_char(operand)),
            '7' => Op::Cdv(COp::from_char(operand)),
            _ => panic!("unknown combo op"),
        }
    }

    fn run(&self, cpu: &mut Cpu) -> usize {
        match self {
            Op::Adv(cop) => {
                let a_ind = Reg::A.index();
                let a = cpu.registers[a_ind];
                let x = match cop {
                    COp::Num(n) => 2_u64.pow(*n as u32),
                    COp::Reg(reg) => 2_u64.pow(cpu.registers[reg.index()] as u32),
                };
                cpu.registers[a_ind] = a / x;
                1
            }
            Op::Bxl(lit_op) => {
                let b_ind = Reg::B.index();
                let b = cpu.registers[b_ind];
                let x = b.bitxor(lit_op.0 as u64);
                cpu.registers[b_ind] = x;
                1
            }
            Op::Bst(cop) => {
                let b_ind = Reg::B.index();
                let x = match cop {
                    COp::Num(n) => (*n as u64) % 8,
                    COp::Reg(reg) => cpu.registers[reg.index()] % 8,
                };
                cpu.registers[b_ind] = x;
                1
            }
            Op::Jnz(lit_op) => {
                let a_ind = Reg::A.index();
                let a = cpu.registers[a_ind];
                if a != 0 {
                    cpu.pointer = (lit_op.0 / 2) as usize;
                    0
                } else {
                    1
                }
            }
            Op::Bxc => {
                let b_ind = Reg::B.index();
                let c_ind = Reg::C.index();
                let b = cpu.registers[b_ind];
                let c = cpu.registers[c_ind];
                let x = b.bitxor(c);
                cpu.registers[b_ind] = x;
                1
            }
            Op::Out(cop) => {
                let x = match cop {
                    COp::Num(n) => (*n as u64) % 8,
                    COp::Reg(reg) => cpu.registers[reg.index()] % 8,
                };
                cpu.output.push(x);
                1
            }
            Op::Bdv(cop) => {
                let b_ind = Reg::A.index();
                let a_ind = Reg::A.index();
                let a = cpu.registers[a_ind];
                let x = match cop {
                    COp::Num(n) => 2_u64.pow(*n as u32),
                    COp::Reg(reg) => 2_u64.pow(cpu.registers[reg.index()] as u32),
                };
                cpu.registers[b_ind] = a / x;
                1
            }
            Op::Cdv(cop) => {
                let c_ind = Reg::C.index();
                let a_ind = Reg::A.index();
                let a = cpu.registers[a_ind];
                let x = match cop {
                    COp::Num(n) => 2_u64.pow(*n as u32),
                    COp::Reg(reg) => 2_u64.pow(cpu.registers[reg.index()] as u32),
                };
                cpu.registers[c_ind] = a / x;
                1
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LitOp(u8);

impl LitOp {
    fn from_char(c: char) -> Self {
        match c {
            '0' => LitOp(0),
            '1' => LitOp(1),
            '2' => LitOp(2),
            '3' => LitOp(3),
            '4' => LitOp(4),
            '5' => LitOp(5),
            '6' => LitOp(6),
            '7' => LitOp(7),
            _ => panic!("unknown combo op"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum COp {
    Num(u8),
    Reg(Reg),
}

impl COp {
    fn from_char(c: char) -> Self {
        match c {
            '0' => COp::Num(0),
            '1' => COp::Num(1),
            '2' => COp::Num(2),
            '3' => COp::Num(3),
            '4' => COp::Reg(Reg::A),
            '5' => COp::Reg(Reg::B),
            '6' => COp::Reg(Reg::C),
            _ => panic!("unknown combo op"),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let parts = input.blocks();
    let registers = parts[0]
        .lines()
        .map(|l| {
            l.split_once(": ")
                .expect("Should parse register")
                .1
                .parse::<u64>()
                .expect("Register should contain number")
        })
        .collect_vec();
    let program = parts[1]
        .split_once(": ")
        .expect("Program should parse")
        .1
        .split(",")
        .map(|s| s.chars().next().expect("Splist shoudl be chars"))
        .collect_vec();
    let program = program
        .chunks(2)
        .map(|cs| Op::from_chars(cs[0], cs[1]))
        .collect_vec();
    let cpu = Cpu {
        program,
        pointer: 0,
        output: Vec::new(),
        registers: [registers[0], registers[1], registers[2]],
    };
    let res = cpu.run_program();
    Some(res.into_iter().join(","))
}

fn find_initial_a<F>(curr: u64, index: usize, run: F, target: &[u64]) -> Option<u64>
where
    F: Fn(u64) -> Vec<u64> + Clone,
{
    if index == 0 {
        return Some(curr);
    }
    let next_index = index - 1;
    // check for each 3 bit value
    for n in 0..8 {
        // the program itself divides A by 8 every loop
        let new_curr = curr * 8 + n;
        let res = run(new_curr);
        if res[0] == target[next_index] {
            // if we find match at the target position, try for next position with value
            let new_val = find_initial_a(new_curr, next_index, run.clone(), target);
            if let Some(n) = new_val {
                return Some(n);
            }
        }
    }
    // match not found
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = input.blocks();
    let registers = parts[0]
        .lines()
        .map(|l| {
            l.split_once(": ")
                .expect("Should parse register")
                .1
                .parse::<u64>()
                .expect("Register should contain number")
        })
        .collect_vec();
    let program_str = parts[1]
        .split_once(": ")
        .expect("Program should parse")
        .1
        .trim();
    let program_literals = program_str
        .split(",")
        .map(|s| s.parse::<u64>().expect("Should parse literals"))
        .collect_vec();
    let program = program_str
        .split(",")
        .map(|s| s.chars().next().expect("Splits should be chars"))
        .collect_vec();
    let program = program
        .chunks(2)
        .map(|cs| Op::from_chars(cs[0], cs[1]))
        .collect_vec();
    let run = move |new_a: u64| {
        let cpu = Cpu {
            program: program.clone(),
            pointer: 0,
            output: Vec::new(),
            registers: [new_a, registers[1], registers[2]],
        };
        cpu.run_program()
    };
    find_initial_a(0, program_literals.len(), run, &program_literals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
