#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("03a: {a}");
    println!("03b: {b}");
}

#[inline(never)]
fn solve(contents: &str) -> (u32, u32) {
    let mut checklist = [0_u8; 0x100];

    let mut score1 = 0;
    let mut score2 = 0;

    let mut i = 1;

    for line in contents.lines() {
        let n = line.len();
        let (a, b) = line.split_at(n / 2);

        for byte in a.as_bytes() {
            checklist[*byte as usize] |= i;
        }

        for byte in b.as_bytes() {
            if checklist[*byte as usize] & i != 0 {
                if *byte > 96 {
                    score1 += (*byte - 96) as u32;
                } else {
                    score1 += (*byte - 38) as u32;
                }
                break;
            }
        }

        for byte in b.as_bytes() {
            checklist[*byte as usize] |= i;
        }

        if i == 4 {
            for byte in line.as_bytes() {
                if checklist[*byte as usize] == 7 {
                    if *byte > 96 {
                        score2 += (*byte - 96) as u32;
                    } else {
                        score2 += (*byte - 38) as u32;
                    }
                    break;
                }
            }
            checklist.fill(0);
            i = 1;
        } else {
            i <<= 1;
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
