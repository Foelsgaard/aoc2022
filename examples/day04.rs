use aoc2022::get_input;

fn main() {
    let contents = get_input();

    let mut score1 = 0;
    let mut score2 = 0;

    for line in contents.lines() {
        let mut endpoints = line
            .split(',')
            .flat_map(|x| x.split('-'))
            .flat_map(|x| x.parse::<usize>().ok());

        let a = endpoints.next().unwrap();
        let b = endpoints.next().unwrap();
        let c = endpoints.next().unwrap();
        let d = endpoints.next().unwrap();

        score1 += (a <= c && b >= d || c <= a && d >= b) as usize;
        score2 += (b >= c && a <= d) as usize;
    }

    println!("04a: {}", score1);
    println!("04b: {}", score2);
}
