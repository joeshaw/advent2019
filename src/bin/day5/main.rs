use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Halt,
}

impl TryFrom<usize> for Op {
    type Error = String;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
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

impl TryFrom<usize> for Mode {
    type Error = String;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(format!("Unknown mode {}", v)),
        }
    }
}

struct Program {
    mem: Vec<usize>,
    pc: usize,
}

impl Program {
    fn new(mem: Vec<usize>) -> Self {
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
            Mode::Position => self.mem[self.pc + offset],
            Mode::Immediate => self.pc + offset,
        };
        loc
    }

    fn operand(&mut self, offset: usize) -> usize {
        let loc = self.loc(offset);
        self.mem[loc]
    }

    fn run(&mut self, _input: i32) {
        loop {
            match self.opcode().unwrap() {
                Op::Add => {
                    let result = self.operand(1) + self.operand(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result;
                    self.pc += 4;
                }
                Op::Mul => {
                    let result = self.operand(1) * self.operand(2);
                    let offset = self.loc(3);
                    self.mem[offset] = result;
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
    let program: Vec<usize> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // First challenge
    let mut newprog = program.to_vec();
    newprog[1] = 12;
    newprog[2] = 2;
    let mut p = Program::new(newprog);
    p.run(1);
    println!("{}", p.mem[0]);

    // Second challenge
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut newprog = program.to_vec();
            newprog[1] = noun;
            newprog[2] = verb;
            let mut p = Program::new(newprog);
            p.run(1);
            if p.mem[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
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
    }
}
