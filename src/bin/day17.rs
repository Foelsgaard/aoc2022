#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;
use core::num;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("17a: {a}");
    println!("17b: {b}");
}

struct Solver<'a> {
    dxs: &'a [bool],
    dx_ix: usize,

    buffer: [u8; 0x100],
    height: num::Wrapping<u8>,
    floor: num::Wrapping<u8>,
    wrapping: bool,
    round: usize,

    total_height: u128,
}

impl<'a> Solver<'a> {
    fn new(dxs: &'a [bool]) -> Self {
        Self {
            dxs,
            dx_ix: 0,

            buffer: [0; 0x100],
            height: num::Wrapping(0),
            floor: num::Wrapping(0),
            wrapping: false,
            round: 0,

            total_height: 0,
        }
    }

    fn reset(&mut self) {
        self.dx_ix = 0;
        self.buffer.fill(0);
        self.height = num::Wrapping(0);
        self.floor = num::Wrapping(0);
        self.wrapping = false;
        self.round = 0;
        self.total_height = 0;
    }

    fn step(&mut self) -> u8 {
        let rock_type = [
            [0b1111, 0b0000, 0b0000, 0b0000],
            [0b0010, 0b0111, 0b0010, 0b0000],
            [0b0111, 0b0001, 0b0001, 0b0000],
            [0b0001, 0b0001, 0b0001, 0b0001],
            [0b0011, 0b0011, 0b0000, 0b0000],
        ];

        let rock_width = [4, 3, 3, 1, 2];

        let Self {
            dxs,
            dx_ix,
            buffer,
            height,
            floor,
            wrapping,
            round,
            total_height,
        } = self;

        let rock_ix = *round % 5;
        *round += 1;

        let mut x: u8 = 5 - rock_width[rock_ix];
        let mut y = *height + num::Wrapping(3_u8);

        let mut added_height = 0;

        loop {
            let typ = rock_type[rock_ix];

            let x1 = if dxs[*dx_ix] {
                x.saturating_sub(1)
            } else {
                (7 - rock_width[rock_ix]).min(x + 1)
            };

            *dx_ix = (*dx_ix + 1) % dxs.len();

            let mut py = y;

            let b0 = (typ[0] << x1) & buffer[py.0 as usize];
            py += 1;
            let b1 = (typ[1] << x1) & buffer[py.0 as usize];
            py += 1;
            let b2 = (typ[2] << x1) & buffer[py.0 as usize];
            py += 1;
            let b3 = (typ[3] << x1) & buffer[py.0 as usize];

            #[cfg(any())]
            {
                print!("Round {}", round);
                if *wrapping {
                    println!(" WRAPPING");
                } else {
                    println!();
                }
                println!("Total height: {}", total_height);

                let mut fy = *height;
                fy += 4;
                for i in 0..=0x10 {
                    fy -= 1;
                    let row = buffer[fy.0 as usize];
                    let rock_row = if fy == y {
                        typ[0]
                    } else if fy == y + num::Wrapping(1) {
                        typ[1]
                    } else if fy == y + num::Wrapping(2) {
                        typ[2]
                    } else if fy == y + num::Wrapping(3) {
                        typ[3]
                    } else {
                        0
                    } << x;

                    for r in 0..7 {
                        if rock_row & (1 << (6 - r)) != 0 {
                            print!("@");
                        } else if row & (1 << (6 - r)) != 0 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    if i == 0 {
                        println!(" ({}, {})", x, y);
                    } else {
                        println!();
                    }
                }
                println!();
            }

            if b0 | b1 | b2 | b3 == 0 {
                x = x1;
            }

            py -= 3;
            let b4 = (typ[1] << x) & buffer[py.0 as usize];
            py -= 1;
            let b5 = (typ[0] << x) & buffer[py.0 as usize];

            if y != *floor && (b4 | b5) == 0 {
                y -= 1;
            } else {
                let mut py = y;
                for mut row in typ {
                    row <<= x;
                    buffer[py.0 as usize] |= row;
                    let dh = (row != 0 && buffer[py.0 as usize] == row) as u8;
                    added_height += dh;
                    *height += dh;
                    let (new_floor, overflow) = height.0.overflowing_add(8);
                    *wrapping |= overflow;
                    if *wrapping {
                        *floor = num::Wrapping(new_floor);
                        let mut fy = *floor;
                        fy -= 1;
                        buffer[fy.0 as usize] = 0;
                    }
                    py += 1;
                }

                break;
            }
        }

        *total_height += added_height as u128;

        added_height
    }
}

fn solve(contents: &str) -> (usize, u128) {
    let mut dxs = [false; 0x10000];
    let mut dxs_len = 0;

    for ch in contents.chars() {
        let dx = match ch {
            '<' => false,
            '>' => true,
            _ => continue,
        };
        dxs[dxs_len] = dx;
        dxs_len += 1;
    }

    let mut score1 = 0;
    let mut solver1 = Solver::new(&dxs[..dxs_len]);
    for _ in 0..2022 {
        score1 += solver1.step() as usize;
    }

    // Tortoise and Hare for cycle detection
    let mut tort_solver = Solver::new(&dxs[..dxs_len]);
    let mut hare_solver = Solver::new(&dxs[..dxs_len]);

    let mut lam = 1_u128;
    let mut mu = 0_u128;

    let mut tort: u128 = 0;
    let mut hare: u128 = 0;

    tort <<= 8;
    tort |= tort_solver.step() as u128;
    hare <<= 8;
    hare |= hare_solver.step() as u128;
    hare <<= 8;
    hare |= hare_solver.step() as u128;

    while tort != hare {
        tort <<= 8;
        tort |= tort_solver.step() as u128;
        hare <<= 8;
        hare |= hare_solver.step() as u128;
        hare <<= 8;
        hare |= hare_solver.step() as u128;
    }

    tort_solver.reset();
    tort = 0;
    while tort != hare {
        tort <<= 8;
        tort |= tort_solver.step() as u128;
        hare <<= 8;
        hare |= hare_solver.step() as u128;
        mu += 1;
    }

    hare = tort;
    hare_solver = tort_solver;
    hare <<= 8;
    hare |= hare_solver.step() as u128;

    while tort != hare {
        hare <<= 8;
        hare |= hare_solver.step() as u128;
        lam += 1;
    }

    let n = (1_000_000_000_000 - mu) / lam;
    let k = (1_000_000_000_000 - mu) % lam;

    let mut a = 0;
    let mut b = 0;
    let mut solver = hare_solver;
    solver.reset();
    for _ in 0..mu {
        b += solver.step() as u128;
    }

    for _ in 0..k {
        a += solver.step() as u128;
    }

    b += a;

    for _ in 0..lam - k {
        a += solver.step() as u128;
    }

    let score2 = n * a + b;

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day17");

        let (a, b) = solve(contents);

        assert_eq!(a, 3137);
        assert_eq!(b, 1564705882327);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day17");

        bencher.iter(|| solve(contents));
    }
}
