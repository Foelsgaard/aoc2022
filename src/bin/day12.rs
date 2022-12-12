#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("12a: {}", a);
    println!("12b: {}", b);
}

const GRID_SIZE: usize = 0x50;

fn solve(contents: &str) -> (usize, usize) {
    let mut grid = [0_u8; GRID_SIZE * GRID_SIZE];
    let mut pot_starts = [0; GRID_SIZE * GRID_SIZE];
    let mut pot_start_len = 0;

    let mut starti = 0;
    let mut endi = 0;

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    contents.lines()
        .flat_map(|line| line.as_bytes())
        .enumerate()
        .for_each(|(i, b)| {
            if *b == 'S' as u8 {
                starti = i;
                grid[i] = 'a' as u8;
                pot_starts[pot_start_len] = i;
                pot_start_len += 1;
            } else if *b == 'E' as u8 {
                endi = i;
                grid[i] = 'z' as u8;
            } else if *b == 'a' as u8 {
                grid[i] = *b;
                pot_starts[pot_start_len] = i;
                pot_start_len += 1;
            } else {
                grid[i] = *b;
            }
        });

    let score1 = least_steps(&grid, width, height, starti, endi);

    let mut score2 = usize::MAX;
    for starti in &pot_starts[..pot_start_len] {
        let n = least_steps(&grid, width, height, *starti, endi);

        if n < score2 {
            score2 = n;
        }
    }

    (score1, score2)
}

fn least_steps(grid: &[u8], width: usize, height: usize, starti: usize, endi: usize) -> usize {
    let heu = |x0, y0, x1, y1| {
        let dx: isize = x0 - x1;
        let dy: isize = y0 - y1;
        (dx.abs() + dy.abs()) as usize
    };

    let x0 = (starti % width) as isize;
    let y0 = (starti / width) as isize;
    let x1 = (endi % width) as isize;
    let y1 = (endi / width) as isize;

    let mut came_from = [usize::MAX; GRID_SIZE * GRID_SIZE];
    let mut g_score = [usize::MAX; GRID_SIZE * GRID_SIZE];
    let mut f_score = [usize::MAX; GRID_SIZE * GRID_SIZE];

    g_score[starti] = 0;
    f_score[starti] = heu(x0, y0, x1, y1);

    let mut queue = [0; GRID_SIZE * GRID_SIZE];
    let mut qlen = 1;

    queue[0] = starti;

    let mut current;

    loop {
        queue[..qlen].sort_by(|i, j| f_score[*j].cmp(&f_score[*i]));

        if qlen == 0 {
            return usize::MAX;
        }
        current = queue[qlen - 1];
        qlen -= 1;

        if current == endi {
            break;
        }

        let x = (current % width) as isize;
        let y = (current / width) as isize;

        'n_loop: for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= width as isize {
                continue;
            }
            if ny < 0 || ny >= height as isize {
                continue;
            }

            let next = (ny * (width as isize) + nx) as usize;
            if grid[current] < grid[next] && grid[next] - grid[current] > 1 {
                continue;
            }

            let tentative_g_score = g_score[current] + 1;
            if tentative_g_score < g_score[next] {
                came_from[next] = current;
                g_score[next] = tentative_g_score;
                f_score[next] = tentative_g_score + heu(nx, ny, x1, y1);

                for i in &queue[..qlen] {
                    if *i == next {
                        continue 'n_loop;
                    }
                }
                queue[qlen] = next;
                qlen += 1;
            }
        }
    }

    let mut steps = 0;

    while current != starti {
        current = came_from[current];
        steps += 1;
    }

    steps

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day12");

        let (a, b) = solve(contents);

        assert_eq!(a, 361);
        assert_eq!(b, 354);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day12");

        bencher.iter(|| {
            solve(contents);
        });
    }
}
