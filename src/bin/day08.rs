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

    let grid_width = contents.lines().next().unwrap().len();

    let mut n = 0;
    for ch in contents.chars() {
        match ch {
            '\n' => {},
            _ => {
                grid[n] = (ch as u8) - 48;
                n += 1;
            }
        }
    }

    let mut score1 = 0;
    let mut score2 = 0;
    for i in 0..n {
        let h = grid[i];
        let x0 = i % grid_width;
        let y0 = i / grid_width;

        let mut v = [
            grid_width - x0 - 1,
            x0,
            grid_width - y0 - 1,
            y0
        ];

        let mut visible = 4;
    
        let mut d = 1;
        for x in x0 + 1..grid_width {
            let j = y0 * grid_width + x;

            if h <= grid[j] {
                v[0] = d;
                visible -= 1;
                break;
            }

            d += 1;
        }

        d = 1;
        for x in (0..x0).rev() {
            let j = y0 * grid_width + x;

            if h <= grid[j] {
                v[1] = d;
                visible -= 1;
                break;
            }

            d += 1;
        }

        d = 1;
        for y in y0 + 1..grid_width {
            let j = y * grid_width + x0;

            if h <= grid[j] {
                v[2] = d;
                visible -= 1;
                break;
            }

            d += 1;
        }

        d = 1;
        for y in (0..y0).rev() {
            let j = y * grid_width + x0;

            if h <= grid[j] {
                v[3] = d;
                visible -= 1;
                break;
            }

            d += 1;
        }

        let s = v.iter().product();

        score1 += (visible > 0) as usize;
        score2 = score2.max(s);
    }

    (score1, score2)
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

        bencher.iter(|| {
            solve(contents)
        });
    }
}
