#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("03a: {}", a);
    println!("03b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut score1 = 0;

    for line in contents.lines() {
        let n = line.len();
        let mut checklist = [false; 52];

        let (a, b) = line.split_at(n / 2);

        for c in a.chars() {
            if b.contains(c) {
                let pri = (c as u8).checked_sub(96).unwrap_or((c as u8) - 38) as usize;
                if !checklist[pri - 1] {
                    score1 += pri;
                    checklist[pri - 1] = true;
                }
            }
        }
    }

    let mut score2 = 0;
    let mut lines = contents.lines();

    loop {
        let mut checklist = [false; 52];

        let a = if let Some(line) = lines.next() {
            line
        } else {
            break;
        };
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                let pri = (ch as u8).checked_sub(96).unwrap_or((ch as u8) - 38) as usize;

                if !checklist[pri - 1] {
                    score2 += pri;
                    checklist[pri - 1] = true;
                }
            }
        }
    }

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day03");

        let (a, b) = solve(contents);

        assert_eq!(a, 7795);
        assert_eq!(b, 2703);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day03");

        bencher.iter(|| solve(contents));
    }
}
