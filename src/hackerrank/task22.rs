use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pickingNumbers(a: &[i32]) -> i32 {
    let mut freq = [0; 101];

    for &num in a {
        freq[num as usize] += 1;
    }

    let mut max_len = 0;
    for i in 1..=100 {
        let current_len = freq[i] + freq[i - 1];
        if current_len > max_len {
            max_len = current_len;
        }
    }

    max_len
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pickingNumbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
