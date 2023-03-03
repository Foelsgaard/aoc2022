#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::read_input;

fn main() {
    let mut buf = [0; 0x10000];
    let contents = read_input(&mut buf);

    let (a, b) = solve(contents);

    println!("01a: {a}");
    println!("01b: {b}");
}

#[inline(never)]
fn solve(contents: &str) -> (u32, u32) {
    let bytes = contents.as_bytes();
    let n = bytes.len();

    let mut totals = [0, 0, 0, 0];

    let mut i = 0;
    let mut x = 0;

    while i < n {
        let b = bytes[i];
        if b == b'\n' {
            if x == 0 {
                totals.sort_unstable();
                totals[0] = 0;
            } else {
                totals[0] += x;
                x = 0;
            }
        } else {
            x *= 10;
            x += (b - 48) as u32;
        }

        i += 1;
    }

    totals.sort_unstable();

    let a = totals[3];
    let b = totals[1] + totals[2] + totals[3];

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
