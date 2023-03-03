#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;
use core::str::FromStr;
use core::{fmt, ops};

fn main() {
    let contents = get_input();

    let a = solve(&contents);

    println!("25a: {a}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snafu<const N: usize>([i8; N]);

impl<const N: usize> Snafu<N> {
    fn zero() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> ops::AddAssign for Snafu<N> {
    fn add_assign(&mut self, other: Self) {
        let mut carry = 0;
        for i in 0..N {
            match self.0[i] + other.0[i] + carry {
                -5 => {
                    self.0[i] = 0;
                    carry = -1;
                }
                -4 => {
                    self.0[i] = 1;
                    carry = -1;
                }
                -3 => {
                    self.0[i] = 2;
                    carry = -1;
                }
                3 => {
                    self.0[i] = -2;
                    carry = 1;
                }
                4 => {
                    self.0[i] = -1;
                    carry = 1;
                }
                5 => {
                    self.0[i] = 0;
                    carry = 1;
                }
                x if (-2..=2).contains(&x) => {
                    self.0[i] = x;
                    carry = 0;
                }
                _ => panic!(),
            }
        }
    }
}

impl<const N: usize> fmt::Display for Snafu<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut len = 0;
        for i in 0..N {
            if self.0[i] != 0 {
                len = i;
            }
        }
        len += 1;

        for i in 0..len {
            match self.0[len - i - 1] {
                -2 => write!(f, "=")?,
                -1 => write!(f, "-")?,
                0 => write!(f, "0")?,
                1 => write!(f, "1")?,
                2 => write!(f, "2")?,
                _ => panic!(),
            }
        }

        Ok(())
    }
}

impl<const N: usize> FromStr for Snafu<N> {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ns = [0; N];

        let len = s.len();
        for (i, ch) in s.chars().enumerate() {
            match ch {
                '=' => {
                    ns[len - i - 1] = -2;
                }
                '-' => {
                    ns[len - i - 1] = -1;
                }
                '0' => {}
                '1' => {
                    ns[len - i - 1] = 1;
                }
                '2' => {
                    ns[len - i - 1] = 2;
                }
                ch => {
                    return Err(ch);
                }
            }
        }
        Ok(Self(ns))
    }
}

fn solve(contents: &str) -> Snafu<20> {
    let mut n = Snafu::<20>::zero();
    for line in contents.lines() {
        n += line.parse().unwrap();
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day25");

        let a = solve(contents);

        assert_eq!(a, "20==1==12=0111=2--20".parse::<Snafu<20>>().unwrap());
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day25");

        bencher.iter(|| solve(contents));
    }
}
