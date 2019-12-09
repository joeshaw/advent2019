fn num_to_vec(mut n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

fn check_part1(v: Vec<i32>) -> bool {
    let mut repeated = false;
    for i in 0..v.len() - 1 {
        if v[i] < v[i + 1] {
            return false;
        }
        if v[i] == v[i + 1] {
            repeated = true
        }
    }
    repeated
}

fn check_part2(v: Vec<i32>) -> bool {
    let mut has_double = false;
    let mut n = 1;
    for i in 0..v.len() - 1 {
        if v[i] < v[i + 1] {
            return false;
        }

        if v[i] == v[i + 1] {
            n += 1;
        } else {
            if n == 2 {
                has_double = true;
            }
            n = 1;
        }
    }
    has_double || n == 2
}

fn main() {
    let part1 = (109165..=576723)
        .filter(|x| check_part1(num_to_vec(*x)))
        .count();
    println!("{}", part1);

    let part2 = (109165..=576723)
        .filter(|x| check_part2(num_to_vec(*x)))
        .count();
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests = vec![(111111, true), (223450, false), (123789, false)];
        for (v, expected) in tests {
            assert_eq!(check_part1(num_to_vec(v)), expected);
        }
    }

    #[test]
    fn test_part2() {
        let tests = vec![
            (112233, true),
            (123444, false),
            (111122, true),
            (223333, true),
        ];
        for (v, expected) in tests {
            assert_eq!(check_part2(num_to_vec(v)), expected);
        }
    }
}
