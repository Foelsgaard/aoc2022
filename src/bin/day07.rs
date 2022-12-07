#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("07a: {}", a);
    println!("07b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut lines = contents.lines().peekable();
    lines.next();
    
    let mut total_size = 0;
    let root_size = dir_size(&mut lines, "/", 0, &mut total_size, &mut 0);

    let mut lines = contents.lines().peekable();
    lines.next();

    let mut deletable = usize::MAX;
    let free_space = 70_000_000 - root_size;
    dir_size(&mut lines, "/", free_space, &mut 0, &mut deletable);

    (total_size, deletable)
}

fn dir_size<'a, I>(lines: &mut I, _dir: &str, free_space: usize, total_size: &mut usize, deletable: &mut usize) -> usize
where I: Iterator<Item = &'a str> {
    let mut size = 0;

    while let Some(line) = lines.next() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(new_dir) = cmd.strip_prefix("cd ") {
                match new_dir {
                    ".." => {
                        break;
                    }
                    _ => {
                        size += dir_size(lines, new_dir, free_space, total_size, deletable);
                    }
                }
            }
        } else if let Some(_) = line.strip_prefix("dir ") {
        } else {
            let (file_size, _file_name) = line.split_once(' ').unwrap();
            size += file_size.parse::<usize>().unwrap();
        }
    }

    if size <= 100_000 {
        *total_size += size;
    }

    if size + free_space >= 30_000_000 && *deletable > size {
        *deletable = size;
    }

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

        bencher.iter(|| {
            solve(contents)
        });
    }
}
