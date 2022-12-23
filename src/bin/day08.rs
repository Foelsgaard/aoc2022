#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("08a: {}", a);
    println!("08b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut grid = [0_u8; 0x10000];
    let mut visible = [false; 0x10000];
    let mut scenic = [1; 0x10000];

    let grid_width = contents.lines().next().unwrap().len();

    let mut n = 0;
    for ch in contents.chars() {
        match ch {
            '\n' => {}
            _ => {
                grid[n] = (ch as u8) - 48;
                n += 1;
            }
        }
    }

    let mut calc = DistCalc::new();

    for j in 1..grid_width - 1 {
        let a = j * grid_width;
        let b = a + grid_width;
        let row_iter = grid[a..b].iter().copied();

        let col_iter = (0..grid_width)
            .into_iter()
            .map(|i| grid[i * grid_width + j]);

        calc.distances(row_iter, |k, d, v| {
            let i = a + k;
            visible[i] |= v;
            scenic[i] *= d;
        });
        calc.distances(col_iter, |k, d, v| {
            let i = k * grid_width + j;
            visible[i] |= v;
            scenic[i] *= d;
        });
    }

    let score1 = visible[..n].iter().filter(|b| **b).count() + 4;
    let score2 = scenic[..n].iter().max().unwrap();

    (score1, *score2)
}

struct DistCalc {
    istack: [usize; 0x80],
    hstack: [u8; 0x80],
}

impl DistCalc {
    fn new() -> Self {
        Self {
            istack: [0; 0x80],
            hstack: [0; 0x80],
        }
    }
    fn distances<I, F>(&mut self, line: I, mut f: F)
    where
        I: IntoIterator<Item = u8>,
        F: FnMut(usize, usize, bool),
    {
        let istack = &mut self.istack;
        let hstack = &mut self.hstack;

        let mut n = 0;
        let mut i = 0;
        for h in line {
            while n > 0 && hstack[n - 1] < h {
                f(istack[n - 1], i - istack[n - 1], false);
                n -= 1;
            }
            let d = if n > 0 { i - istack[n - 1] } else { i };
            f(i, d, n == 0);

            while n > 0 && hstack[n - 1] == h {
                f(istack[n - 1], i - istack[n - 1], false);
                n -= 1;
            }
            istack[n] = i;
            hstack[n] = h;
            n += 1;
            i += 1;
        }

        while n > 0 {
            f(istack[n - 1], i - istack[n - 1] - 1, true);
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day08");

        let (a, b) = solve(contents);

        assert_eq!(a, 1711);
        assert_eq!(b, 301392);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day08");

        bencher.iter(|| solve(contents));
    }
}
