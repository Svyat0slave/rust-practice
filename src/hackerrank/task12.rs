use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp::{min, max};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().copied().reduce(|x, y| lcm(x, y)).unwrap();
    let gcd_b = b.iter().copied().reduce(|x, y| gcd(x, y)).unwrap();

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
