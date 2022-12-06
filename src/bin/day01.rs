#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("01a: {}", a);
    println!("01b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut totals = [0, 0, 0, 0];
    let mut total = 0;

    for line in contents.lines() {
        if line.is_empty() {
            totals[0] = total;
            totals.sort();
            total = 0;
            continue;
        }
        total += line.parse::<usize>().unwrap();
    }
    totals[0] = total;
    totals.sort();

    let a = totals[3];
    let b = totals[1..].iter().sum();

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day01");

        let (a, b) = solve(contents);

        assert_eq!(a, 72718);
        assert_eq!(b, 213089);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day01");

        bencher.iter(|| solve(contents));
    }
}
