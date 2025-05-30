use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let mut jumps = 0;
    let mut i = 0;

    while i < c.len() - 1 {
        if i + 2 < c.len() && c[i + 2] == 0 {
            i += 2;
        } else {
            i += 1;
        }
        jumps += 1;
    }

    jumps
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let c: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
