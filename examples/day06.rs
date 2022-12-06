use aoc2022::get_input;

fn main() {
    let contents = get_input();

    println!("06a: {}", solve(&contents, 4));
    println!("06b: {}", solve(&contents, 14));
}

fn solve(contents: &str, marker_size: usize) -> usize {
    let bytes = contents.as_bytes();

    let mut i = 0;
    let mut checklist = [false; 256];
    for window in bytes.windows(marker_size) {
        for b in window {
            checklist[*b as usize] = true;
        }

        let n = checklist.iter().filter(|x| **x).count();

        if n == marker_size {
            break;
        }

        checklist.fill(false);
        i += 1;
    }

    i + marker_size
}
