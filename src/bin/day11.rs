#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let a = solve(&contents, 20, true);
    let b = solve(&contents, 10000, false);

    println!("11a: {}", a);
    println!("11b: {}", b);
}

#[derive(Clone, Copy)]
enum Arg {
    Const(usize),
    Old,
}

#[derive(Clone, Copy)]
enum Op {
    Add(Arg),
    Mul(Arg),
}

impl Default for Op {
    fn default() -> Op {
        Op::Add(Arg::Const(0))
    }
}

fn solve(contents: &str, rounds: usize, decrease_worry: bool) -> usize {
    const MAX_QUEUE_LENGTH: usize = 0x40;
    const MAX_NUM_QUEUES: usize = 0x10;
    let mut queues = [[0; MAX_QUEUE_LENGTH]; MAX_NUM_QUEUES];
    let mut lengths = [0; MAX_NUM_QUEUES];
    let mut starts = [0; MAX_NUM_QUEUES];
    let mut divisors = [0; MAX_NUM_QUEUES];
    let mut operations = [Op::default(); MAX_NUM_QUEUES];
    let mut targets = [0; MAX_NUM_QUEUES * 2];

    let mut n = 0;
    let mut lines = contents.lines();
    let mut modulus = 1;

    while lines.next().is_some() {
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .flat_map(|n_str| n_str.parse::<usize>().ok());

        for (i, item) in items.enumerate() {
            queues[n][i] = item;
            lengths[n] += 1;
        }

        let mut op_parts = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_ascii_whitespace();

        let op_str = op_parts.next().unwrap();
        let right = match op_parts.next().unwrap() {
            "old" => Arg::Old,
            x => Arg::Const(x.parse::<usize>().unwrap()),
        };

        let op = match op_str {
            "+" => Op::Add(right),
            "*" => Op::Mul(right),
            _ => panic!(),
        };

        operations[n] = op;

        let divisor = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        divisors[n] = divisor;
        modulus *= divisor;

        let tgt_true = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let tgt_false = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        targets[n * 2] = tgt_false;
        targets[n * 2 + 1] = tgt_true;

        lines.next();

        n += 1;
    }

    let mut activity = [0; MAX_NUM_QUEUES];

    for _round in 0..rounds {
        for i in 0..n {
            let a = starts[i];
            let b = a + lengths[i];

            for j in 0..lengths[i] {
                let item = queues[i][a..b][j];
                starts[i] += 1;
                lengths[i] -= 1;
                activity[i] += 1;

                let mut new_item = match operations[i] {
                    Op::Add(Arg::Old) => item + item,
                    Op::Add(Arg::Const(x)) => item + x,
                    Op::Mul(Arg::Old) => item * item,
                    Op::Mul(Arg::Const(x)) => item * x,
                };

                if decrease_worry {
                    new_item /= 3;
                }
                new_item %= modulus;
                let divisible = new_item % divisors[i] == 0;

                let tgt = targets[i * 2 + divisible as usize];

                if starts[tgt] + lengths[tgt] + 1 > MAX_QUEUE_LENGTH {
                    let c = starts[tgt];
                    let d = c + lengths[tgt];
                    queues[tgt].copy_within(c..d, 0);
                    starts[tgt] = 0;
                }
                
                queues[tgt][starts[tgt] + lengths[tgt]] = new_item;
                lengths[tgt] += 1;
            }
        }
    }

    activity[..n].sort();
    let score1 = activity[n - 2..n].iter().copied().product();

    score1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day11");

        let a = solve(&contents, 20, true);
        let b = solve(&contents, 10000, false);

        assert_eq!(a, 120056);
        assert_eq!(b, 21816744824);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day11");

        bencher.iter(|| {
            solve(&contents, 20, true);
            solve(&contents, 10000, false);
        });
    }
}
