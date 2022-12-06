#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("04a: {}", a);
    println!("04b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut score1 = 0;
    let mut score2 = 0;

    for line in contents.lines() {
        let mut endpoints = line
            .split(',')
            .flat_map(|x| x.split('-'))
            .flat_map(|x| x.parse::<usize>().ok());

        let a = endpoints.next().unwrap();
        let b = endpoints.next().unwrap();
        let c = endpoints.next().unwrap();
        let d = endpoints.next().unwrap();

        score1 += (a <= c && b >= d || c <= a && d >= b) as usize;
        score2 += (b >= c && a <= d) as usize;
    }

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day04");

        let (a, b) = solve(contents);

        assert_eq!(a, 459);
        assert_eq!(b, 779);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day04");

        bencher.iter(|| solve(contents));
    }
}
