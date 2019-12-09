use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl TryFrom<i32> for Op {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
            3 => Ok(Op::Input),
            4 => Ok(Op::Output),
            5 => Ok(Op::JumpIfTrue),
            6 => Ok(Op::JumpIfFalse),
            7 => Ok(Op::LessThan),
            8 => Ok(Op::Equals),
            99 => Ok(Op::Halt),
            _ => Err(format!("Unknown opcode {}", v)),
        }
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i32> for Mode {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(format!("Unknown mode {}", v)),
        }
    }
}

struct Program {
    mem: Vec<i32>,
    pc: usize,
}

impl Program {
    fn new(mem: Vec<i32>) -> Self {
        Program { mem: mem, pc: 0 }
    }

    fn opcode(&mut self) -> Result<Op, String> {
        Op::try_from(self.mem[self.pc] % 100)
    }

    fn loc(&mut self, offset: usize) -> usize {
        let mut m = self.mem[self.pc] / 100;
        for _ in 1..offset {
            m /= 10;
        }
        let loc = match Mode::try_from(m % 10).unwrap() {
            Mode::Position => self.mem[self.pc + offset] as usize,
            Mode::Immediate => self.pc + offset,
        };
        loc
    }

    fn get(&mut self, offset: usize) -> i32 {
        let loc = self.loc(offset);
        self.mem[loc]
    }

    fn run(&mut self, input: i32) {
        loop {
            match self
                .opcode()
                .unwrap_or_else(|e| panic!("{} at PC: {}", e, self.pc))
            {
                Op::Add => {
                    let result = self.get(1) + self.get(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result;
                    self.pc += 4;
                }
                Op::Mul => {
                    let result = self.get(1) * self.get(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result;
                    self.pc += 4;
                }
                Op::Input => {
                    let offset = self.loc(1);
                    self.mem[offset] = input;
                    self.pc += 2;
                }
                Op::Output => {
                    let result = self.get(1);
                    println!("{}", result);
                    self.pc += 2;
                }
                Op::JumpIfTrue => {
                    let cond = self.get(1);
                    if cond != 0 {
                        self.pc = self.get(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Op::JumpIfFalse => {
                    let cond = self.get(1);
                    if cond == 0 {
                        self.pc = self.get(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Op::LessThan => {
                    let result = self.get(1) < self.get(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result as i32;
                    self.pc += 4;
                }
                Op::Equals => {
                    let result = self.get(1) == self.get(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result as i32;
                    self.pc += 4;
                }
                Op::Halt => break,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).expect("unable to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to data read from input file");
    let program: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // First challenge
    let mut p = Program::new(program.to_vec());
    p.run(1);

    // Second challenge
    let mut p = Program::new(program.to_vec());
    p.run(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn older_programs() {
        let program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);

        let program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);

        let program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);

        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);
    }

    #[test]
    fn test_run_program() {
        let program = vec![1002, 4, 3, 4, 33];
        let expected = vec![1002, 4, 3, 4, 99];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);

        let program = vec![1101, 100, -1, 4, 0];
        let expected = vec![1101, 100, -1, 4, 99];
        let mut p = Program::new(program);
        p.run(1);
        assert_eq!(p.mem, expected);

        let program = vec![3, 9, 2, 3, 9, 10, 4, 10, 99, 0, 0];
        let expected = vec![3, 9, 2, 3, 9, 10, 4, 10, 99, 5, 15];
        let mut p = Program::new(program);
        p.run(5);
        assert_eq!(p.mem, expected);

        // Test the equals opcode in position mode
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let expected = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8];
        let mut p = Program::new(program);
        p.run(8);
        assert_eq!(p.mem, expected);

        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let expected = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8];
        let mut p = Program::new(program);
        p.run(7);
        assert_eq!(p.mem, expected);

        // Test the less-than opcode in position mode
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let expected = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8];
        let mut p = Program::new(program);
        p.run(8);
        assert_eq!(p.mem, expected);

        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let expected = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8];
        let mut p = Program::new(program);
        p.run(7);
        assert_eq!(p.mem, expected);

        // Test the equals opcode in immediate mode
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let expected = vec![3, 3, 1108, 1, 8, 3, 4, 3, 99];
        let mut p = Program::new(program);
        p.run(8);
        assert_eq!(p.mem, expected);

        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let expected = vec![3, 3, 1108, 0, 8, 3, 4, 3, 99];
        let mut p = Program::new(program);
        p.run(7);
        assert_eq!(p.mem, expected);

        // Test the less-than opcode in immediate mode
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let expected = vec![3, 3, 1107, 0, 8, 3, 4, 3, 99];
        let mut p = Program::new(program);
        p.run(8);
        assert_eq!(p.mem, expected);

        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let expected = vec![3, 3, 1107, 1, 8, 3, 4, 3, 99];
        let mut p = Program::new(program);
        p.run(7);
        assert_eq!(p.mem, expected);
    }
}
