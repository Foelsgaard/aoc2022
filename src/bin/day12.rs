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

const GRID_SIZE: usize = 0x1400;

fn solve(contents: &str) -> (usize, usize) {
    let mut grid = [0_u8; GRID_SIZE];
    let mut pot_starts = [0; GRID_SIZE];
    let mut pot_start_len = 0;

    let mut starti = 0;
    let mut endi = 0;

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    contents
        .lines()
        .flat_map(|line| line.as_bytes())
        .enumerate()
        .for_each(|(i, b)| {
            if *b == b'S' {
                starti = i;
                grid[i] = b'a';
                pot_starts[pot_start_len] = i;
                pot_start_len += 1;
            } else if *b == b'E' {
                endi = i;
                grid[i] = b'z';
            } else if *b == b'a' {
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

fn lookup_insert(set: &mut [usize], key: usize, new: usize) -> usize {
    use core::num::Wrapping;
    let mut i = Wrapping((key & 0xFFFF) as u16);

    while set[i.0 as usize] != usize::MAX && set[i.0 as usize] != key {
        i += 1;
    }

    let old = set[i.0 as usize];
    set[i.0 as usize] = new;
    old
}

const HEAP_SIZE: usize = 0x200;
struct MinHeap<K, V> {
    elems: [(K, V); HEAP_SIZE],
    len: usize,
}

impl<K: Ord + Default + Copy + core::fmt::Debug, V: Default + Copy + core::fmt::Debug>
    MinHeap<K, V>
{
    fn new() -> Self {
        Self {
            elems: [(K::default(), V::default()); HEAP_SIZE],
            len: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let mut index = self.len;
        self.elems[index] = (key, value);
        self.len += 1;

        while index > 0 {
            let parent = (index - 1) >> 1;
            if self.elems[parent].0 < self.elems[index].0 {
                break;
            }

            self.elems.swap(parent, index);
            index = parent;
        }
    }

    fn pop(&mut self) -> Option<(K, V)> {
        use core::mem;
        if self.len == 0 {
            return None;
        }

        let mut index = 0;

        let popped = mem::take(&mut self.elems[index]);
        self.len -= 1;
        self.elems.swap(index, self.len);

        loop {
            let left = (index << 1) + 1;
            let right = (index << 1) + 2;

            let mut min = if left < self.len && self.elems[left].0 < self.elems[index].0 {
                left
            } else {
                index
            };

            if right < self.len && self.elems[right].0 < self.elems[min].0 {
                min = right;
            };

            if min == index {
                break;
            }

            self.elems.swap(index, min);
            index = min;
        }

        Some(popped)
    }
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

    let mut came_from = [usize::MAX; GRID_SIZE];
    let mut g_score = [usize::MAX; GRID_SIZE];
    let mut f_score = [usize::MAX; GRID_SIZE];

    g_score[starti] = 0;
    f_score[starti] = heu(x0, y0, x1, y1);

    let mut set = [usize::MAX; 0x2000];
    let mut queue = MinHeap::new();

    queue.insert(f_score[starti], starti);
    lookup_insert(&mut set, starti, starti);

    let mut current;

    loop {
        match queue.pop() {
            Some((_, v)) => {
                current = v;
            }
            None => {
                return usize::MAX;
            }
        }

        lookup_insert(&mut set, current, usize::MAX);

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

                if lookup_insert(&mut set, next, next) == next {
                    continue 'n_loop;
                }

                queue.insert(f_score[next], next);
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
    fn test_min_heap() {
        let mut heap = MinHeap::new();

        for k in [1243, 2, 347, 9, 0, 4, 1, 243, 58234] {
            heap.insert(k, 0);
        }

        assert_eq!(heap.pop(), Some((0, 0)));
        assert_eq!(heap.pop(), Some((1, 0)));
        assert_eq!(heap.pop(), Some((2, 0)));
        assert_eq!(heap.pop(), Some((4, 0)));
        assert_eq!(heap.pop(), Some((9, 0)));
        assert_eq!(heap.pop(), Some((243, 0)));
        assert_eq!(heap.pop(), Some((347, 0)));
        assert_eq!(heap.pop(), Some((1243, 0)));
        assert_eq!(heap.pop(), Some((58234, 0)));
        assert_eq!(heap.pop(), None);
    }

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
