#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();
    let buffer = &mut [0; 0x10];

    let a = solve(false, &contents, buffer);
    println!("05a: {a}");

    let b = solve(true, &contents, buffer);
    println!("05b: {b}");
}

fn solve<'a>(multi_move: bool, contents: &str, buffer: &'a mut [u8]) -> &'a str {
    const STACK_MAX_WIDTH: usize = 0x10;
    const STACK_MAX_HEIGHT: usize = 0x200;

    let mut stack = ['\0'; STACK_MAX_WIDTH * STACK_MAX_HEIGHT];

    let mut stack_height = 0;
    let stack_width;

    let mut lines = contents.lines().peekable();
    loop {
        let line = lines.next().unwrap();

        if let Some(next_line) = lines.peek() {
            if next_line.is_empty() {
                stack_width = line.split_ascii_whitespace().count();
                break;
            }
        }

        stack_height += 1;
    }

    let mut stack_len = [0; STACK_MAX_WIDTH];
    let mut lines = contents.lines().peekable();

    let mut i = 0;
    let mut j = 0;

    loop {
        let line = lines.next().unwrap();

        if let Some(next_line) = lines.peek() {
            if next_line.is_empty() {
                break;
            }
        }

        let mut ws = 0;

        for ch in line.chars() {
            match ch {
                ' ' => {
                    ws += 1;
                }
                '[' => {
                    i += ws / 4;
                }
                ']' => {
                    ws = 0;
                    i += 1;
                }
                ch => {
                    stack_len[i] = stack_len[i].max(stack_height - j);
                    stack[i * STACK_MAX_HEIGHT + stack_height - j - 1] = ch;
                }
            }
        }

        i = 0;
        j += 1;
    }

    for line in lines.skip(1) {
        let mut tokens = line.split_ascii_whitespace();
        tokens.next();
        let n = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let src = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        tokens.next();
        let dst = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

        let a = STACK_MAX_HEIGHT * src + stack_len[src] - n;
        let b = a + n;
        let c = STACK_MAX_HEIGHT * dst + stack_len[dst];
        let d = c + n;

        stack.copy_within(a..b, c);

        if !multi_move {
            stack[c..d].reverse();
        }

        stack_len[src] -= n;
        stack_len[dst] += n;
    }

    for i in 0..stack_width {
        let n = stack_len[i];
        let ch = stack[STACK_MAX_HEIGHT * i + n - 1];

        buffer[i] = ch as u8;
    }

    std::str::from_utf8(&buffer[..stack_width]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day05");
        let buffer = &mut [0; 0x10];

        let a = solve(false, contents, buffer);
        assert_eq!(a, "JCMHLVGMG");

        let b = solve(true, contents, buffer);
        assert_eq!(b, "LVMRWSSPZ");
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day05");
        let buffer = &mut [0; 0x10];

        bencher.iter(|| {
            solve(false, contents, buffer);
            solve(true, contents, buffer);
        });
    }
}
