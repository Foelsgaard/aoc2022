#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let a = solve::<1, 1>(&contents);
    let b = solve::<811589153, 10>(&contents);

    println!("20a: {}", a);
    println!("20b: {}", b);
}

fn solve<const DEC: i64, const ITER: usize>(contents: &str) -> i64 {
    let mut seq = [0_i64; 5000];
    let mut seqi = [0_usize; 5000];
    let mut slen = 0;

    for line in contents.lines() {
        seq[slen] = line.parse().unwrap();
        seq[slen] *= DEC;
        seqi[slen] = slen;
        slen += 1;
    }

    let mut buf = seqi;

    for _ in 0..ITER {
        for l in &seqi[..slen] {
            let i = buf[..slen].iter().position(|i| i == l).unwrap();
            let k = i as i64 + seq[buf[i]];
            let j = (k.rem_euclid(slen as i64 - 1)) as usize;

            match j.cmp(&i) {
                core::cmp::Ordering::Less => buf[j..=i].rotate_right(1),
                core::cmp::Ordering::Greater => buf[i..=j].rotate_left(1),
                _ => {}
            }
        }
    }

    let i = buf.iter().position(|i| seq[*i] == 0).unwrap();
    let a = seq[buf[(i + 1000) % slen]];
    let b = seq[buf[(i + 2000) % slen]];
    let c = seq[buf[(i + 3000) % slen]];

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day20");

        let a = solve::<1, 1>(&contents);
        let b = solve::<811589153, 10>(&contents);

        assert_eq!(a, 4426);
        assert_eq!(b, 8119137886612);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day20");

        bencher.iter(|| {
            solve::<1, 1>(&contents);
            solve::<811589153, 10>(&contents);
        })
    }
}
