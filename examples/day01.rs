use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let mut totals = [0, 0, 0, 0];
    let mut total = 0;

    for line in contents.lines() {
        if line.is_empty() {
            totals[0] = total;
            totals.sort();
            total = 0;
            continue;
        }
        total += line.parse::<usize>().unwrap();
    }
    totals[0] = total;
    totals.sort();

    println!("01a: {}", totals[3]);
    println!("01b: {}", totals[1..].iter().sum::<usize>());
}
