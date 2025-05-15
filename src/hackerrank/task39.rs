use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn repeatedString(s: &str, n: i64) -> i64 {
    let len = s.len() as i64;
    let count_a_in_s = s.chars().filter(|&c| c == 'a').count() as i64;

    let full_repeats = n / len;
    let remainder = n % len;

    let count_a_in_remainder = s.chars()
        .take(remainder as usize)
        .filter(|&c| c == 'a')
        .count() as i64;

    full_repeats * count_a_in_s + count_a_in_remainder
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();

    let result = repeatedString(&s, n);

    writeln!(&mut fptr, "{}", result).ok();
}
