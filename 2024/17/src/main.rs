use std::collections::HashSet;

use itertools::Itertools;

/// Three bit computer.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct TBC {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
    program: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl TBC {
    fn from_input(input: Vec<String>) -> Self {
        let a = input[0].split(": ").nth(1).unwrap().parse::<i64>().unwrap();
        let b = input[1].split(": ").nth(1).unwrap().parse::<i64>().unwrap();
        let c = input[2].split(": ").nth(1).unwrap().parse::<i64>().unwrap();
        let program = input[4]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        Self {
            a,
            b,
            c,
            pc: 0,
            program,
            output_buffer: Vec::new(),
        }
    }
    fn op(&self) -> i64 {
        *self.program.get(self.pc + 1).unwrap() as i64
    }
    fn combo_op(&self) -> i64 {
        match self.program.get(self.pc + 1).unwrap() {
            n @ 0..=3 => *n as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn step(&mut self) -> Option<()> {
        let ins = self.program.get(self.pc)?;
        match ins {
            0 => self.a /= 2i64.pow(self.combo_op() as u32),
            1 => self.b ^= self.op(),
            2 => self.b = self.combo_op() % 8,
            3 => {
                if self.a != 0 {
                    self.pc = self.op() as usize;
                    return Some(());
                }
            }
            4 => self.b ^= self.c,
            5 => self.output_buffer.push((self.combo_op() % 8) as u8),
            6 => self.b = self.a / 2i64.pow(self.combo_op() as u32),
            7 => self.c = self.a / 2i64.pow(self.combo_op() as u32),
            _ => unreachable!(),
        }
        self.pc += 2;
        Some(())
    }
    fn is_halted(&self) -> bool {
        self.pc >= self.program.len()
    }
    fn format_output(&self) -> String {
        Itertools::intersperse(self.output_buffer.iter().map(|v| (v + b'0') as char), ',')
            .collect::<String>()
    }
}
fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<String> {
    let mut tbc = TBC::from_input(input);
    while !tbc.is_halted() {
        tbc.step();
    }
    Ok(tbc.format_output())
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let corrupt_tbc = TBC::from_input(input);
    // println!("Corrupt machine:\n{corrupt_tbc:?}");
    let mut digits = corrupt_tbc.program.clone();
    digits.reverse();
    let mut prev_bits: Vec<i64> = vec![0];
    let mut next_bits: Vec<i64> = Vec::new();
    for &digit in digits.iter() {
        for &prev_part in &prev_bits {
            for a in 0i64..8 {
                let mut tbc = corrupt_tbc.clone();
                let a_test = (prev_part << 3) + a;
                tbc.a = a_test;
                while !tbc.is_halted() {
                    let _ = tbc.step();
                }
                if *tbc.output_buffer.first().unwrap() == digit {
                    next_bits.push(a_test);
                }
            }
        }
        // println!("The a values: {next_bits:?} produced {digit}");
        prev_bits.clone_from(&next_bits);
        next_bits.truncate(0);
    }
    Ok(prev_bits.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tbc = TBC {
            a: 0,
            b: 0,
            c: 9,
            pc: 0,
            program: vec![2, 6],
            output_buffer: Vec::new(),
        };
        tbc.step();
        let expected = TBC {
            a: 0,
            b: 1,
            c: 9,
            pc: 2,
            program: vec![2, 6],
            output_buffer: Vec::new(),
        };
        assert_eq!(tbc, expected);
    }
    #[test]
    fn test2() {
        let mut tbc = TBC {
            a: 10,
            b: 0,
            c: 0,
            pc: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output_buffer: Vec::new(),
        };
        while !tbc.is_halted() {
            tbc.step();
        }
        assert_eq!(tbc.output_buffer, vec![0, 1, 2]);
    }
    #[test]
    fn test3() {
        let mut tbc = TBC {
            a: 2024,
            b: 0,
            c: 0,
            pc: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output_buffer: Vec::new(),
        };
        while !tbc.is_halted() {
            tbc.step();
        }
        assert_eq!(tbc.output_buffer, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(tbc.a, 0);
    }
    #[test]
    fn test4() {
        let mut tbc = TBC {
            a: 0,
            b: 29,
            c: 0,
            pc: 0,
            program: vec![1, 7],
            output_buffer: Vec::new(),
        };
        tbc.step();
        assert_eq!(tbc.b, 26);
    }
    #[test]
    fn test5() {
        let mut tbc = TBC {
            a: 0,
            b: 2024,
            c: 43690,
            pc: 0,
            program: vec![4, 0],
            output_buffer: Vec::new(),
        };
        tbc.step();
        assert_eq!(tbc.b, 44354);
    }
    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [String::from("4,6,3,5,6,3,5,2,1,0")];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_2.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [117440];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
