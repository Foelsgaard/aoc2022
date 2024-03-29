#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::read_input;

fn main() {
    let mut buf = [0; 0x10000];
    let contents = read_input(&mut buf);
    let (a, b) = solve(contents);

    println!("02a: {a}");
    println!("02b: {b}");
}

fn solve(contents: &str) -> (usize, usize) {
    let bytes = contents.as_bytes();
    let n = bytes.len();
    let mut score1 = 0;
    let mut score2 = 0;

    let mut i = 0;

    while i < n {
        let c1 = bytes[i] as char;
        let c2 = bytes[i + 2] as char;

        let (s1, s2) = match (c1, c2) {
            ('A', 'X') => (4, 3),
            ('A', 'Y') => (8, 4),
            ('A', 'Z') => (3, 8),
            ('B', 'X') => (1, 1),
            ('B', 'Y') => (5, 5),
            ('B', 'Z') => (9, 9),
            ('C', 'X') => (7, 2),
            ('C', 'Y') => (2, 6),
            ('C', 'Z') => (6, 7),
            _ => panic!(),
        };

        score1 += s1;
        score2 += s2;

        i += 4;
    }

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day02");

        let (a, b) = solve(contents);

        assert_eq!(a, 13526);
        assert_eq!(b, 14204);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day02");

        bencher.iter(|| solve(contents));
    }
}
