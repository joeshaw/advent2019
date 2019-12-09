use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut fuel_total = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().expect("expected an integer");
        let mut fuel = fuel_for_mass(mass);
        while fuel > 0 {
            fuel_total += fuel;
            fuel = fuel_for_mass(fuel);
        }
    }

    println!("{}", fuel_total);
}
