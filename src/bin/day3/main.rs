use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;

fn build_wire(path: &str) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();
    let mut p = (0, 0);

    for m in path.trim().split(",") {
        let dir = m.chars().nth(0).unwrap();
        let mut dist: u32 = m[1..].parse().unwrap();

        while dist > 0 {
            p = match dir {
                'L' => (p.0 - 1, p.1),
                'R' => (p.0 + 1, p.1),
                'U' => (p.0, p.1 - 1),
                'D' => (p.0, p.1 + 1),
                _ => panic!("Invalid direction {}", dir),
            };
            vec.push(p);
            dist -= 1;
        }
    }

    vec
}

fn closest_intersection(path1: &str, path2: &str) -> i32 {
    let w1 = build_wire(path1);
    let w2 = build_wire(path2);

    let h1 = HashSet::<(i32, i32)>::from_iter(w1);
    let h2 = HashSet::<(i32, i32)>::from_iter(w2);

    h1.intersection(&h2)
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap()
}

fn shortest_intersection(path1: &str, path2: &str) -> i32 {
    let w1 = build_wire(path1);
    let w2 = build_wire(path2);

    let h1 = HashSet::<(i32, i32)>::from_iter(w1.iter().cloned());
    let h2 = HashSet::<(i32, i32)>::from_iter(w2.iter().cloned());

    h1.intersection(&h2)
        .map(|p| {
            w1.iter().take_while(|x| *x != p).count()
                + w2.iter().take_while(|x| *x != p).count()
                + 2
        })
        .min()
        .unwrap() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("unable to open input file");
    let mut reader = BufReader::new(file);

    let mut path1 = String::new();
    let mut path2 = String::new();

    reader.read_line(&mut path1).unwrap();
    reader.read_line(&mut path2).unwrap();

    println!("closest: {}", closest_intersection(&path1, &path2));
    println!("shortest: {}", shortest_intersection(&path1, &path2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest() {
        let path1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let path2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let expected = 159;

        assert_eq!(closest_intersection(path1, path2), expected);
    }

    #[test]
    fn test_shortest() {
        let path1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let path2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let expected = 610;

        assert_eq!(shortest_intersection(path1, path2), expected);
    }
}
