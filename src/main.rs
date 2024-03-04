use std::io;

fn solution(n: u128, m: u128) -> u128 {
    let difference = m - n;

    let numerator = (m - difference + 1..=m).fold(1, |acc, curr| acc * curr);
    let denominator = (1..=difference).fold(1, |acc, curr| acc * curr);

    numerator / denominator
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<u128>);
        let n = iter.next().unwrap();
        let m = iter.next().unwrap();

        println!("{}", solution(n, m));
    }
}
