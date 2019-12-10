use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tree {
    pairs: HashMap<String, String>,
}

impl Tree {
    fn new(map: Vec<String>) -> Self {
        let mut pairs = HashMap::new();
        for entry in map {
            let mut pair = entry.split(")");
            let parent = pair.next().unwrap();
            let child = pair.next().unwrap();
            pairs.insert(child.to_string(), parent.to_string());
        }
        Tree { pairs }
    }

    fn num_orbits(&self, id: String) -> i32 {
        let mut id = id;
        let mut count = 0;
        loop {
            match self.pairs.get(&id) {
                Some(parent) => {
                    count += 1;
                    id = parent.to_string();
                }
                None => break,
            }
        }
        count
    }

    fn total_orbits(&self) -> i32 {
        self.pairs
            .keys()
            .fold(0, |acc, x| acc + self.num_orbits(x.to_string()))
    }

    fn path(&self, id: String) -> Vec<String> {
        let mut id = id;
        let mut v = Vec::new();
        loop {
            match self.pairs.get(&id) {
                Some(parent) => {
                    v.push(parent.to_string());
                    id = parent.to_string();
                }
                None => break,
            }
        }
        v.reverse();
        v
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("unable to open input file");
    let reader = BufReader::new(file);

    let t = Tree::new(reader.lines().map(|x| x.unwrap()).collect());

    // part 1
    println!("{}", t.total_orbits());

    // part 2
    let you_path = t.path("YOU".to_string());
    let san_path = t.path("SAN".to_string());

    // Walk along common ancestors
    let common = you_path
        .iter()
        .zip(&san_path)
        .take_while(|x| *x.0 == *x.1)
        .count();

    // Add the path difference from san_path
    let transfers = you_path.len() + san_path.len() - 2 * common;

    println!("{}", transfers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let map = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let t = Tree::new(map.iter().map(|s| s.to_string()).collect());
        assert_eq!(t.num_orbits("D".to_string()), 3);
        assert_eq!(t.num_orbits("L".to_string()), 7);
        assert_eq!(t.num_orbits("COM".to_string()), 0);

        assert_eq!(t.total_orbits(), 42);

        assert_eq!(t.path("D".to_string()), vec!["COM", "B", "C"]);
    }
}
