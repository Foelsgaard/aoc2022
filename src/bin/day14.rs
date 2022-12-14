#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let a = solve::<false>(&contents);
    let b = solve::<true>(&contents);

    println!("14a: {}", a);
    println!("14b: {}", b);
}

const GRID_WIDTH: usize = 0x200;
const GRID_HEIGHT: usize = 0x100;

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Sand,
    Rock,
}

fn solve<const FLOOR: bool>(contents: &str) -> usize {
    let mut grid = [Tile::Air; GRID_WIDTH * GRID_HEIGHT];
    let mut minx = [500, 0];
    let mut maxx = [500, 0];

    let lines = contents.lines();

    let mut xs = [[0_isize, 0_isize]; 0x800];
    let mut xlen = 0;
    let mut rline_len = [0; 0x800];
    let mut num_rlines = 0;

    for line in lines {
        let vertices = line.split(" -> ").flat_map(|vertex| vertex.split_once(','));

        let mut n = 0;
        for v_str in vertices {
            let x: [isize; 2] = [v_str.0.parse().unwrap(), v_str.1.parse().unwrap()];
            minx[0] = minx[0].min(x[0]);
            minx[1] = minx[1].min(x[1]);
            maxx[0] = maxx[0].max(x[0]);
            maxx[1] = maxx[1].max(x[1]);

            xs[xlen] = x;
            xlen += 1;
            n += 1;
        }

        rline_len[num_rlines] = n;
        num_rlines += 1;
    }

    let mut rlines = xs[..xlen].windows(2);
    for len in rline_len.iter().take(num_rlines) {
        for _ in 0..len - 1 {
            let line = rlines.next().unwrap();
            let mut x0 = line[0];
            let x1 = line[1];

            let dx = [(x1[0] - x0[0]).signum(), (x1[1] - x0[1]).signum()];

            while x0 != x1 {
                let ix = sub2ind(x0, minx);
                grid[ix] = Tile::Rock;

                x0[0] += dx[0];
                x0[1] += dx[1];
            }
            let ix = sub2ind(x0, minx);
            grid[ix] = Tile::Rock;
        }
        rlines.next();
    }

    let mut score = 0;

    'outer: loop {
        let mut x = [500, 0];

        loop {
            let down = [x[0], x[1] + 1];
            let left = [x[0] - 1, x[1] + 1];
            let right = [x[0] + 1, x[1] + 1];

            if FLOOR && down[1] == maxx[1] + 2 {
                let ix = sub2ind(x, minx);
                grid[ix] = Tile::Sand;
                score += 1;
                break;
            }

            let down_ix = sub2ind(down, minx);
            let left_ix = sub2ind(left, minx);
            let right_ix = sub2ind(right, minx);

            if matches!(grid[down_ix], Tile::Air) {
                x = down;
            } else if matches!(grid[left_ix], Tile::Air) {
                x = left;
            } else if matches!(grid[right_ix], Tile::Air) {
                x = right;
            } else {
                let ix = sub2ind(x, minx);
                grid[ix] = Tile::Sand;
                score += 1;
                if FLOOR && x == [500, 0] {
                    break 'outer;
                }
                break;
            }

            if !FLOOR && x[1] > maxx[1] {
                break 'outer;
            }
        }
    }

    score
}

fn sub2ind(x: [isize; 2], minx: [isize; 2]) -> usize {
    let ix = (x[1] - minx[1]) * GRID_WIDTH as isize + x[0] - minx[0];
    ix as usize
}

fn _println_grid<const FLOOR: bool>(grid: &[Tile], minx: [isize; 2], maxx: [isize; 2]) {
    use Tile::*;

    for y in minx[1]..=maxx[1] + 1 {
        for x in minx[0]..=maxx[0] {
            let ix = sub2ind([x, y], minx);
            match grid[ix] {
                Air => print!("."),
                Rock => print!("#"),
                Sand => print!("O"),
            }
        }
        println!();
    }
    if FLOOR {
        for _ in minx[0]..=maxx[0] {
            print!("#");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day14");

        let a = solve::<false>(contents);
        let b = solve::<true>(contents);

        assert_eq!(a, 1016);
        assert_eq!(b, 25402);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day14");

        bencher.iter(|| {
            solve::<false>(contents);
            solve::<true>(contents);
        });
    }
}
