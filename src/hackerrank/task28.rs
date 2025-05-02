use std::io::{self, Write};

fn reverse_number(n: i32) -> i32 {
    let reversed_str: String = n.abs().to_string().chars().rev().collect();
    reversed_str.parse::<i32>().unwrap()
}

fn beautifulDays(i: i32, j: i32, k: i32) -> i32 {
    let mut count = 0;
    for day in i..=j {
        let rev_day = reverse_number(day);
        if (day - rev_day).abs() % k == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read input
    stdin.read_line(&mut input).unwrap();
    let params: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let i = params[0];
    let j = params[1];
    let k = params[2];

    // Call beautifulDays function and print result
    let result = beautifulDays(i, j, k);
    println!("{}", result);
}
