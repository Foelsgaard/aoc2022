use aoc2022::get_input;

fn main() {
    let contents = get_input();

    println!("06a: {}", solve(&contents, 4));
    println!("06b: {}", solve(&contents, 14));
}

fn solve(contents: &str, marker_size: usize) -> usize {
    let bytes = contents.as_bytes();

    let mut n = 0;
    let mut checklist = [0_u8; 256];
    for b in bytes.iter().take(marker_size) {
        let check = &mut checklist[*b as usize];
        *check += 1;
        n += (*check == 1) as u8;
    }

    let mut i: usize = 0;
    let mut j: usize = marker_size;
    while n as usize != marker_size {
        let a = bytes[i];
        let b = bytes[j];

        let check_a = &mut checklist[a as usize];
        *check_a -= 1;
        n -= (*check_a == 0) as u8;

        let check_b = &mut checklist[b as usize];
        *check_b += 1;
        n += (*check_b == 1) as u8;

        i += 1;
        j += 1;
    }

    j
}
