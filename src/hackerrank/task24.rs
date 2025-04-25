use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn hurdleRace(k: i32, height: &[i32]) -> i32 {
    let max_hurdle = *height.iter().max().unwrap_or(&0);
    if max_hurdle > k {
        max_hurdle - k
    } else {
        0
    }
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

    let height: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = hurdleRace(k, &height);

    writeln!(&mut fptr, "{}", result).ok();
}
