use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let mut score = 0;

    for line in contents.lines() {
        let n = line.len();
        let mut checklist = [false; 52];

        let (a, b) = line.split_at(n / 2);

        for c in a.chars() {
            if b.contains(c) {
                let pri = (c as u8).checked_sub(96).unwrap_or((c as u8) - 38) as usize;
                if !checklist[pri - 1] {
                    score += pri;
                    checklist[pri - 1] = true;
                }
            }
        }
    }
    println!("{}", score);

    let mut score = 0;
    let mut lines = contents.lines();

    loop {
        let mut checklist = [false; 52];

        let a = if let Some(line) = lines.next() {
            line
        } else {
            break;
        };
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                let pri = (ch as u8).checked_sub(96).unwrap_or((ch as u8) - 38) as usize;

                if !checklist[pri - 1] {
                    score += pri;
                    checklist[pri - 1] = true;
                }
            }
        }
    }

    println!("{}", score);
}
