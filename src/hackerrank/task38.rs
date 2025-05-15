use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn nonDivisibleSubset(k: i32, s: &[i32]) -> i32 {
    let mut counts = vec![0; k as usize];

    for &num in s {
        counts[(num % k) as usize] += 1;
    }

    let mut result = 0;

    if counts[0] > 0 {
        result += 1;
    }

    for i in 1..=(k / 2) {
        let i = i as usize;
        let opposite = (k as usize) - i;

        if i == opposite {
            result += 1;
        } else {
            result += counts[i].max(counts[opposite]);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = nonDivisibleSubset(k, &s);

    writeln!(&mut fptr, "{}", result).ok();
}
