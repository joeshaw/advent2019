use std::env;
use std::fs::File;
use std::io::prelude::*;

fn run_program(program: &mut Vec<usize>) {
    let mut iptr: usize = 0;
    loop {
        match program[iptr] {
            1 => {
                let offsets = (program[iptr + 1], program[iptr + 2], program[iptr + 3]);
                let result = program[offsets.0] + program[offsets.1];
                program[offsets.2] = result;
                iptr += 4;
            }
            2 => {
                let offsets = (program[iptr + 1], program[iptr + 2], program[iptr + 3]);
                let result = program[offsets.0] * program[offsets.1];
                program[offsets.2] = result;
                iptr += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode {} at position {}", program[iptr], iptr),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).expect("unable to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to read data from input file");
    let program: Vec<usize> = contents
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // First challenge
    let mut newprog = program.to_vec();
    newprog[1] = 12;
    newprog[2] = 2;
    run_program(&mut newprog);
    println!("{}", newprog[0]);

    // Second challenge
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut newprog = program.to_vec();
            newprog[1] = noun;
            newprog[2] = verb;
            run_program(&mut newprog);
            if newprog[0] == 19690720 {
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
    fn test_run_program() {
        let mut program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        run_program(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        run_program(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        run_program(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        run_program(&mut program);
        assert_eq!(program, expected);
    }
}
