#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

use core::cmp::Ordering;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("13a: {}", a);
    println!("13b: {}", b);
}

fn solve(contents: &str) -> (usize, usize) {
    let mut lines = contents.lines();

    let mut score1 = 0;
    let mut i = 1;

    let mut buf = [""; 0x400];
    buf[0] = "[[2]]";
    buf[1] = "[[6]]";
    let mut len = 2;
    while let Some(line0) = lines.next() {
        let line1 = lines.next().unwrap();

        let mut tokens0 = Tokenizer::tokenize(line0);
        let mut tokens1 = Tokenizer::tokenize(line1);

        let ord = compare_packets(&mut tokens0, &mut tokens1, [None, None]);
        let correct = matches!(ord, Ordering::Less);
        if correct {
            score1 += i;
        }

        lines.next();
        i += 1;

        buf[len] = line0;
        len += 1;
        buf[len] = line1;
        len += 1;
    }

    buf[..len].sort_unstable_by(|line0, line1| {
        let mut tokens0 = Tokenizer::tokenize(line0);
        let mut tokens1 = Tokenizer::tokenize(line1);

        compare_packets(&mut tokens0, &mut tokens1, [None, None])
    });

    let mut score2 = 0;

    let mut i = 1;
    for line in &buf[..len] {
        if *line == "[[2]]" {
            score2 += i;
        } else if *line == "[[6]]" {
            score2 *= i;
        }
        i += 1;
    }

    (score1, score2)
}

#[derive(Debug)]
enum Token {
    LStart,
    LEnd,
    Int(u8),
}

struct Tokenizer<I> {
    end: bool,
    chars: I,
}

impl<'a> Tokenizer<std::str::Chars<'a>> {
    fn tokenize(s: &'a str) -> Self {
        Self {
            end: false,
            chars: s.chars(),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if self.end {
            self.end = false;
            return Some(LEnd);
        }

        let mut ch = self.chars.next();
        if let Some(',') = ch {
            ch = self.chars.next();
        }

        match ch {
            Some('[') => Some(LStart),
            Some(']') => Some(LEnd),
            Some(x) if x.is_ascii_digit() => {
                let mut n = x as u8 - 0x30;
                for y in self.chars.by_ref() {
                    match y {
                        ',' => {
                            break;
                        }
                        ']' => {
                            self.end = true;
                            break;
                        }
                        y if y.is_ascii_digit() => {
                            n *= 10;
                            n += y as u8 - 0x30;
                        }
                        _ => panic!(),
                    }
                }
                Some(Int(n))
            }
            None => None,
            ch => panic!("{:?}", ch),
        }
    }
}

fn compare_packets<I>(ts0: &mut I, ts1: &mut I, mut cap: [Option<Token>; 2]) -> Ordering
where
    I: Iterator<Item = Token>,
{
    use Token::*;

    let mut b = [cap[0].is_some() as u8 * 2, cap[1].is_some() as u8 * 2];
    loop {
        let t0 = if b[0] == 1 {
            LEnd
        } else {
            cap[0].take().or_else(|| ts0.next()).unwrap()
        };
        let t1 = if b[1] == 1 {
            LEnd
        } else {
            cap[1].take().or_else(|| ts1.next()).unwrap()
        };
        b[0] = b[0].saturating_sub(1);
        b[1] = b[1].saturating_sub(1);

        match (t0, t1) {
            (Int(left), Int(right)) => {
                if left == right {
                    continue;
                }

                return left.cmp(&right);
            }
            (LStart, LStart) => {
                let ord = compare_packets(ts0, ts1, [None, None]);
                if matches!(ord, Ordering::Equal) {
                    continue;
                }

                return ord;
            }
            (LStart, Int(right)) => {
                let ord = compare_packets(ts0, ts1, [None, Some(Int(right))]);
                if matches!(ord, Ordering::Equal) {
                    continue;
                }

                return ord;
            }
            (Int(left), LStart) => {
                let ord = compare_packets(ts0, ts1, [Some(Int(left)), None]);
                if matches!(ord, Ordering::Equal) {
                    continue;
                }

                return ord;
            }
            (LEnd, LEnd) => {
                return Ordering::Equal;
            }
            (LEnd, _) => {
                return Ordering::Less;
            }
            (_, LEnd) => {
                return Ordering::Greater;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day13");

        let (a, b) = solve(contents);

        assert_eq!(a, 5529);
        assert_eq!(b, 27690);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day13");

        bencher.iter(|| {
            solve(contents);
        });
    }
}
