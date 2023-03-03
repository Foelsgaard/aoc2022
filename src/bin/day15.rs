#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let (a, b) = solve(&contents);

    println!("15a: {a}");
    println!("15b: {b}");
}

const BUF_SIZE: usize = 0x20;

fn solve(contents: &str) -> (usize, isize) {
    let mut xs = [0; BUF_SIZE];
    let mut ys = [0; BUF_SIZE];
    let mut rs = [0; BUF_SIZE];
    let mut is = [0; BUF_SIZE];
    let mut rects = [[[0, 0], [0, 0]]; BUF_SIZE];

    let mut len = 0;

    for line in contents.lines() {
        let mut ns = line
            .split_ascii_whitespace()
            .flat_map(|s| s.split(':'))
            .flat_map(|s| s.split(','))
            .flat_map(|s| s.split('='))
            .flat_map(|s| s.parse::<isize>());

        let sx = ns.next().unwrap();
        let sy = ns.next().unwrap();
        let bx = ns.next().unwrap();
        let by = ns.next().unwrap();

        let r = (bx - sx).abs() + (by - sy).abs();

        xs[len] = sx;
        ys[len] = sy;
        rs[len] = r;
        is[len] = len;

        let x0 = sx - r;
        let x1 = sx + r;
        let y0 = sy;
        let y1 = sy;

        let [q0, r0] = cart2hex(x0, sy);
        let [q1, r1] = cart2hex(x1, sy);

        assert_eq!(hex2cart(q0, r0), [x0, y0]);
        assert_eq!(hex2cart(q1, r1), [x1, y1]);

        rects[len] = [[q0, r0], [q1 + 1, r1 + 1]];

        len += 1;
    }

    let target_row = 2_000_000;

    is[..len].sort_unstable_by_key(|i| {
        let dy = (ys[*i] - target_row).abs();
        let r = rs[*i];
        let x = xs[*i];

        x - r + dy
    });

    let mut score1 = 0;

    let mut prev_x1 = isize::MIN;
    for i in &is[..len] {
        let dy = (ys[*i] - target_row).abs();
        let r = rs[*i];
        let x = xs[*i];

        if dy > r {
            continue;
        }

        let x0 = (x - r + dy).max(prev_x1);
        let x1 = x + r - dy;

        let dx = x1 - x0;

        if dx > 0 {
            score1 += (x1 - x0) as usize;
            prev_x1 = x1;
        }
    }

    let mut p0 = [isize::MAX, isize::MAX];
    let mut p1 = [isize::MIN, isize::MIN];

    for rect in &rects[..len] {
        let [[q0, r0], [q1, r1]] = *rect;
        p0[0] = p0[0].min(q0);
        p0[1] = p0[1].min(r0);
        p1[0] = p1[0].max(q1);
        p1[1] = p1[1].max(r1);
    }

    let mut stack0 = [[[0, 0], [0, 0]]; 0x80];
    let mut stack1 = [[[0, 0], [0, 0]]; 0x80];
    stack0[0] = [p0, p1];
    let mut stacklen0 = 1;
    let mut stacklen1 = 0;

    for splitter in &rects[..len] {
        for target in &stack0[..stacklen0] {
            for rect in split_rect_by_rect(*splitter, *target) {
                stack1[stacklen1] = rect;
                stacklen1 += 1;
            }
        }

        stacklen0 = stacklen1;
        stacklen1 = 0;

        core::mem::swap(&mut stack0, &mut stack1);
    }

    let mut score2 = 0;

    for rect in &stack0[..stacklen0] {
        let [[q0, r0], [q1, r1]] = rect;

        if q1 - q0 == 1 && r1 - r0 == 1 {
            let [x, y] = hex2cart(*q0, *r0);
            score2 = x * 4_000_000 + y;
            break;
        }
    }

    (score1, score2)
}

fn split_rect_by_rect(splitter: Rect, target: Rect) -> RectSplitIter {
    RectSplitIter {
        quadrant: 0,
        rect0: splitter,
        rect1: target,
    }
}

type Point = [isize; 2];
type Rect = [Point; 2];

struct RectSplitIter {
    quadrant: u8,
    rect0: Rect,
    rect1: Rect,
}

impl Iterator for RectSplitIter {
    type Item = Rect;

    fn next(&mut self) -> Option<Rect> {
        let [[x0, y0], [x1, y1]] = self.rect0;
        let [[x2, y2], [x3, y3]] = self.rect1;

        loop {
            self.quadrant += 1;

            let [x4, x5, y4, y5] = match self.quadrant {
                1 => [x0.max(x2), x3, y2, y0.min(y3)],
                2 => [x1.max(x2), x3, y0.max(y2), y3],

                3 => [x2, x1.min(x3), y1.max(y2), y3],

                4 => [x2, x0.min(x3), y2, y1.min(y3)],
                _ => {
                    return None;
                }
            };

            if x5 > x4 && y5 > y4 {
                return Some([[x4, y4], [x5, y5]]);
            }
        }
    }
}

fn cart2hex(x: isize, y: isize) -> [isize; 2] {
    let q = -y;
    let r = x + y;

    let col = 2 * q + r;
    let row = r;
    [col, row]
}

fn hex2cart(col: isize, row: isize) -> [isize; 2] {
    let q = (col - row) / 2;
    let r = row;

    let x = r + q;
    let y = -q;

    [x, y]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let contents = include_str!("../../input/day15");

        let (a, b) = solve(contents);

        assert_eq!(a, 5256611);
        assert_eq!(b, 13337919186981);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(bencher: &mut Bencher) {
        let contents = include_str!("../../input/day15");

        bencher.iter(|| {
            solve(contents);
        });
    }
}
