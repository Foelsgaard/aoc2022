#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("18a: {}", a);
    println!("18b: {}", b);
}

const GRID_SIZE: usize = 30 * 30 * 30;

fn solve(contents: &str) -> (usize, usize) {
    let mut grid = [false; GRID_SIZE];
    let mut cubes = [0; GRID_SIZE];
    let mut clen = 0;

    for line in contents.lines() {
        let mut coords = line.split(',').flat_map(|n| n.parse::<usize>());

        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();

        let ix = z * 30 * 30 + y * 30 + x;
        grid[ix] = true;
        cubes[clen] = ix;
        clen += 1;
    }

    let mut score1 = 0;

    for ix in &cubes[..clen] {
        let x = ix % 30;
        let y = (ix % 900) / 30;
        let z = ix / 900;

        let mut n = 6;
        if x != 0 {
            n -= grid[ix - 1] as usize;
        }
        if x != 29 {
            n -= grid[ix + 1] as usize;
        }
        if y != 0 {
            n -= grid[ix - 30] as usize;
        }
        if y != 29 {
            n -= grid[ix + 30] as usize;
        }
        if z != 0 {
            n -= grid[ix - 900] as usize;
        }
        if z != 29 {
            n -= grid[ix + 900] as usize;
        }

        score1 += n;
    }

    let mut score2 = score1;
    let mut visited = [false; GRID_SIZE];
    let mut outside = [false; GRID_SIZE];

    let mut stack = [0; GRID_SIZE];
    let mut slen = 1;

    while slen > 0 {
        slen -= 1;
        let ix = stack[slen];

        if grid[ix] {
            continue;
        }

        visited[ix] = true;
        outside[ix] = true;

        let x = ix % 30;
        let y = (ix % 900) / 30;
        let z = ix / 900;

        if x != 0 && !visited[ix - 1] {
            stack[slen] = ix - 1;
            slen += 1;
            visited[ix - 1] = true;
        }
        if x != 29 && !visited[ix + 1] {
            stack[slen] = ix + 1;
            slen += 1;
            visited[ix + 1] = true;
        }
        if y != 0 && !visited[ix - 30] {
            stack[slen] = ix - 30;
            slen += 1;
            visited[ix - 30] = true;
        }
        if y != 29 && !visited[ix + 30] {
            stack[slen] = ix + 30;
            slen += 1;
            visited[ix + 30] = true;
        }
        if z != 0 && !visited[ix - 900] {
            stack[slen] = ix - 900;
            slen += 1;
            visited[ix - 900] = true;
        }
        if z != 29 && !visited[ix + 900] {
            stack[slen] = ix + 900;
            slen += 1;
            visited[ix + 900] = true;
        }
    }

    for ix in 0..GRID_SIZE {
        if !outside[ix] && !grid[ix] {
            score2 -= *grid.get(ix - 1).unwrap_or(&false) as usize;
            score2 -= *grid.get(ix + 1).unwrap_or(&false) as usize;
            score2 -= *grid.get(ix - 30).unwrap_or(&false) as usize;
            score2 -= *grid.get(ix + 30).unwrap_or(&false) as usize;
            score2 -= *grid.get(ix - 900).unwrap_or(&false) as usize;
            score2 -= *grid.get(ix + 900).unwrap_or(&false) as usize;
        }
    }

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day18");

        let (a, b) = solve(contents);

        assert_eq!(a, 3364);
        assert_eq!(b, 2006);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day18");

        bencher.iter(|| solve(contents));
    }
}
