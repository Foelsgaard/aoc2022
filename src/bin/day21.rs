#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("21a: {a}");
    println!("21b: {b}");
}

#[derive(Clone, Copy)]
enum KNode<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Lit(i64),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Lit(i64),
}

const MAX_NODES: usize = 0x2000;

fn solve(contents: &str) -> (i64, i64) {
    let mut ht_keys = [""; MAX_NODES];
    let mut ht_values = [KNode::Lit(0); MAX_NODES];
    let mut num_ops = 0;

    for line in contents.lines() {
        let (key, op_str) = line.split_once(':').unwrap();

        use KNode::*;
        let node = match op_str.trim().parse::<i64>() {
            Ok(n) => Lit(n),
            Err(_) => {
                let mut op_parts = op_str.trim().split_ascii_whitespace();
                let a = op_parts.next().unwrap();
                let op = op_parts.next().unwrap();
                let b = op_parts.next().unwrap();

                match op {
                    "+" => Add(a, b),
                    "-" => Sub(a, b),
                    "*" => Mul(a, b),
                    "/" => Div(a, b),
                    _ => panic!(),
                }
            }
        };

        let hash = hash(key);
        let mut i = hash % MAX_NODES;
        while !ht_keys[i].is_empty() && ht_keys[i] != key {
            i = (i + 1) % MAX_NODES;
        }
        ht_keys[i] = key;
        ht_values[i] = node;
        num_ops += 1;
    }

    let mut seq = [Op::Lit(0); MAX_NODES];

    order_ops(&ht_keys, &ht_values, "root", &mut seq);

    let score1 = eval(&seq[..num_ops]);
    let i = num_ops - 1;
    seq[i] = Op::Sub;
    let j = seq_ix(&ht_keys, &ht_values, "root", "humn").unwrap();

    seq[j] = Op::Lit(0);
    let x = eval(&seq[..num_ops]) < 0;
    let mut y = x;
    let mut offset = 1;
    let (mut a, mut b) = loop {
        seq[j] = Op::Lit(offset);
        if (eval(&seq[..num_ops]) < 0) != x {
            y = true;
            break (0, offset);
        }
        seq[j] = Op::Lit(-offset);
        if (eval(&seq[..num_ops]) < 0) != x {
            break (-offset, 0);
        }
        offset <<= 1;
    };

    seq[j] = Op::Lit(a);
    seq[j] = Op::Lit(b);

    while a != b {
        let c = (a + b) / 2;
        seq[j] = Op::Lit(c);

        let n = eval(&seq[..num_ops]);
        if n == 0 {
            a = c;
            break;
        }
        if (eval(&seq[..num_ops]) < 0) == x {
            if y {
                a = c;
            } else {
                b = c;
            }
        } else if y {
            b = c;
        } else {
            a = c;
        }
    }

    while {
        seq[j] = Op::Lit(a - 1);
        eval(&seq[..num_ops]) == 0
    } {
        a -= 1;
    }

    let score2 = a;

    (score1, score2)
}

fn eval(seq: &[Op]) -> i64 {
    let mut stack = [0; 0x2000];
    let mut slen = 0;

    for op in seq {
        use Op::*;
        match op {
            Add => {
                slen -= 1;
                let b = stack[slen];
                let a = stack[slen - 1];
                stack[slen - 1] = a + b;
            }
            Sub => {
                slen -= 1;
                let b = stack[slen];
                let a = stack[slen - 1];
                stack[slen - 1] = a - b;
            }
            Mul => {
                slen -= 1;
                let b = stack[slen];
                let a = stack[slen - 1];
                stack[slen - 1] = a * b;
            }
            Div => {
                slen -= 1;
                let b = stack[slen];
                let a = stack[slen - 1];
                stack[slen - 1] = a / b;
            }
            Lit(n) => {
                stack[slen] = *n;
                slen += 1;
            }
        }
    }

    stack[0]
}

fn hash(key: &str) -> usize {
    key.chars().map(|ch| ch as usize).product::<usize>()
}

fn seq_ix(ht_keys: &[&str], ht_values: &[KNode], key: &str, tgt: &str) -> Result<usize, usize> {
    let hash = hash(key);
    let mut i = hash % MAX_NODES;
    while !ht_keys[i].is_empty() && ht_keys[i] != key {
        i = (i + 1) % MAX_NODES;
    }

    if key == tgt {
        return Ok(0);
    }

    use KNode::*;
    match ht_values[i] {
        Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
            let n = match seq_ix(ht_keys, ht_values, a, tgt) {
                Ok(j) => {
                    return Ok(j);
                }
                Err(n) => n,
            };
            let m = match seq_ix(ht_keys, ht_values, b, tgt) {
                Ok(j) => {
                    return Ok(j + n);
                }
                Err(m) => m,
            };
            Err(n + m + 1)
        }
        Lit(_) => Err(1),
    }
}

fn order_ops(ht_keys: &[&str], ht_values: &[KNode], key: &str, seq: &mut [Op]) -> usize {
    let hash = hash(key);
    let mut i = hash % MAX_NODES;
    while !ht_keys[i].is_empty() && ht_keys[i] != key {
        i = (i + 1) % MAX_NODES;
    }

    use KNode::*;
    match ht_values[i] {
        Add(a, b) => {
            let n = order_ops(ht_keys, ht_values, a, seq);
            let m = order_ops(ht_keys, ht_values, b, &mut seq[n..]);
            seq[n + m] = Op::Add;
            n + m + 1
        }
        Sub(a, b) => {
            let n = order_ops(ht_keys, ht_values, a, seq);
            let m = order_ops(ht_keys, ht_values, b, &mut seq[n..]);
            seq[n + m] = Op::Sub;
            n + m + 1
        }
        Mul(a, b) => {
            let n = order_ops(ht_keys, ht_values, a, seq);
            let m = order_ops(ht_keys, ht_values, b, &mut seq[n..]);
            seq[n + m] = Op::Mul;
            n + m + 1
        }
        Div(a, b) => {
            let n = order_ops(ht_keys, ht_values, a, seq);
            let m = order_ops(ht_keys, ht_values, b, &mut seq[n..]);
            seq[n + m] = Op::Div;
            n + m + 1
        }
        Lit(n) => {
            seq[0] = Op::Lit(n);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day21");

        let (a, b) = solve(contents);

        assert_eq!(a, 43699799094202);
        assert_eq!(b, 3375719472770);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day21");

        bencher.iter(|| solve(contents));
    }
}
