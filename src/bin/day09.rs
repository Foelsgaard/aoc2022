#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let a = solve(&contents, 2);
    let b = solve(&contents, 10);

    println!("09a: {}", a);
    println!("09b: {}", b);
}

fn solve(contents: &str, rope_length: usize) -> usize {
    const GRID_SIZE: usize = 0x400;
    const OFFSET: isize = (GRID_SIZE as isize) / 2;

    let mut grid = [false; GRID_SIZE * GRID_SIZE];
    let mut x = [0; 0x20];
    let mut y = [0; 0x20];

    let mut score = 0;

    for line in contents.lines() {
        let (dir, n_str) = line.split_once(' ').unwrap();
        let n = n_str.parse::<isize>().unwrap();

        let (mut dx, mut dy);

        match dir {
            "R" => {
                dx = n;
                dy = 0;
            }
            "U" => {
                dx = 0;
                dy = -n;
            }
            "D" => {
                dx = 0;
                dy = n;
            }
            "L" => {
                dx = -n;
                dy = 0;
            }
            _ => panic!(),
        }

        while dx != 0 || dy != 0 {
            x[0] += dx.signum();
            dx -= dx.signum();
            y[0] += dy.signum();
            dy -= dy.signum();

            for i in 0..rope_length - 1 {
                let x0 = x[i];
                let x1 = &mut x[i + 1];
                let y0 = y[i];
                let y1 = &mut y[i + 1];

                if (x0 - *x1).abs() > 1 && (y0 - *y1).abs() > 1 {
                    *x1 += (x0 - *x1).signum();
                    *y1 += (y0 - *y1).signum();
                } else if x0 - *x1 > 1 {
                    if y0 != *y1 {
                        *y1 = y0;
                    }
                    *x1 = x0 - 1;
                } else if x0 - *x1 < -1 {
                    if y0 != *y1 {
                        *y1 = y0;
                    }
                    *x1 = x0 + 1;
                }
                if y0 - *y1 > 1 {
                    if x0 != *x1 {
                        *x1 = x0;
                    }
                    *y1 = y0 - 1;
                } else if y0 - *y1 < -1 {
                    if x0 != *x1 {
                        *x1 = x0;
                    }
                    *y1 = y0 + 1;
                }
            }

            let tx = x[rope_length - 1];
            let ty = y[rope_length - 1];
            let ti = ((ty + OFFSET) * (GRID_SIZE as isize) + (tx + OFFSET)) as usize;

            score += !grid[ti] as usize;
            grid[ti] = true;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day09");

        let a = solve(contents, 2);
        let b = solve(contents, 10);

        assert_eq!(a, 5779);
        assert_eq!(b, 2331);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day09");

        bencher.iter(|| {
            solve(contents, 2);
            solve(contents, 10);
        });
    }
}
