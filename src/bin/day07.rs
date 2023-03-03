#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("07a: {a}");
    println!("07b: {b}");
}

fn solve(contents: &str) -> (usize, usize) {
    let mut lines = contents.lines();
    lines.next();

    let mut score1 = 0;
    let root_size = traverse(&mut lines, "/", &mut |size| {
        if size <= 100_000 {
            score1 += size;
        }
    });

    let mut lines = contents.lines();
    lines.next();

    let mut score2 = usize::MAX;
    let free_space = 70_000_000 - root_size;
    traverse(&mut lines, "/", &mut |size| {
        if size + free_space >= 30_000_000 && score2 > size {
            score2 = size;
        }
    });

    (score1, score2)
}

fn traverse<'a, I, F>(lines: &mut I, _dir: &str, f: &mut F) -> usize
where
    I: Iterator<Item = &'a str>,
    F: FnMut(usize),
{
    let mut size = 0;

    while let Some(line) = lines.next() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(new_dir) = cmd.strip_prefix("cd ") {
                match new_dir {
                    ".." => {
                        break;
                    }
                    _ => {
                        size += traverse(lines, new_dir, f);
                    }
                }
            }
        } else if line.strip_prefix("dir ").is_some() {
        } else {
            let (file_size, _file_name) = line.split_once(' ').unwrap();
            size += file_size.parse::<usize>().unwrap();
        }
    }

    f(size);

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day07");

        let (a, b) = solve(contents);

        assert_eq!(a, 1491614);
        assert_eq!(b, 6400111);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day07");

        bencher.iter(|| solve(contents));
    }
}
