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
    let mut count = [0; 256];
    let mut checklist = [false; 256];

    let mut score1 = 0;
    for line in contents.lines() {
        let n = line.len();
        let (a, b) = line.split_at(n / 2);

        for byte in a.as_bytes() {
            checklist[*byte as usize] = true;
        }

        for byte in b.as_bytes() {
            if checklist[*byte as usize] {
                count[*byte as usize] += 1;
                break;
            }
        }

        checklist.fill(false);
    }

    for (ch, n) in count.iter().enumerate().skip(38) {
        let pri = (ch as u8).checked_sub(96).unwrap_or((ch as u8) - 38) as usize;

        score1 += pri * n;
    }

    count.fill(0);

    let mut checklist = [0_u8; 256];
    let mut score2 = 0;
    let mut lines = contents.lines();

    while let Some(a) = lines.next() {
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        for byte in a.as_bytes() {
            checklist[*byte as usize] = 1;
        }

        for byte in b.as_bytes() {
            if checklist[*byte as usize] == 1 {
                checklist[*byte as usize] = 2;
            }
        }

        for byte in c.as_bytes() {
            if checklist[*byte as usize] == 2 {
                count[*byte as usize] += 1;
                break;
            }
        }

        checklist.fill(0);
    }

    for (ch, n) in count.iter().enumerate().skip(38) {
        let pri = (ch as u8).checked_sub(96).unwrap_or((ch as u8) - 38) as usize;

        score2 += pri * n;
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
