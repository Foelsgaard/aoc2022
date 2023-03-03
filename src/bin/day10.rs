#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();
    let mut buf = [0; 0x10];

    let (a, b) = solve(&contents, &mut buf);

    println!("10a: {a}");
    println!("10b: {b}");
}

fn solve<'a>(contents: &str, out_buf: &'a mut [u8]) -> (isize, &'a str) {
    let lines = contents.lines();

    let mut letters = [0_u32; 8];

    let mut score = 0;
    let mut cycle = 1;
    let mut x = 1;

    let mut inc_score = |cycle: usize, x: isize| {
        let row = (cycle - 1) / 40;
        let col = (cycle - 1) % 40;
        if (x - 1..=x + 1).contains(&(col as isize)) {
            let letter = &mut letters[col / 5];
            let bit = row * 5 + col % 5;
            *letter |= 1 << bit;
        }
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            score += x * (cycle as isize);
        }
    };

    inc_score(cycle, x);

    for line in lines {
        if line.starts_with("noop") {
            cycle += 1;
            inc_score(cycle, x);
            continue;
        }

        let n = line
            .strip_prefix("addx ")
            .unwrap()
            .parse::<isize>()
            .unwrap();

        cycle += 1;
        inc_score(cycle, x);
        x += n;
        cycle += 1;
        inc_score(cycle, x);
    }

    for (i, letter) in letters.iter().enumerate() {
        let ch = match letter {
            0b010010100101111010010100100110 => 'A',
            0b011110000100001001110000101111 => 'E',
            0b011100100101101000010100100110 => 'G',
            0b010010100101001011110100101001 => 'H',
            0b001100100101000010000100001100 => 'J',
            0b010010010100101000110010101001 => 'K',
            0b011110000100001000010000100001 => 'L',
            0b000010000100111010010100100111 => 'P',
            0b010010010100111010010100100111 => 'R',
            0b001100100101001010010100101001 => 'U',
            0b011110000100010001000100001111 => 'Z',
            // TODO(jfo): Add more letters
            _ => {
                panic!("{i}: {letter:b}");
            }
        };

        out_buf[i] = ch as u8;
    }

    let display = std::str::from_utf8(&out_buf[..8]).unwrap();

    (score, display)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day10");
        let mut buf = [0; 0x10];

        let (a, b) = solve(contents, &mut buf);

        assert_eq!(a, 15360);
        assert_eq!(b, "PHLHJGZA");
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day10");
        let mut buf = [0; 0x10];

        bencher.iter(|| {
            solve(contents, &mut buf);
        });
    }
}
