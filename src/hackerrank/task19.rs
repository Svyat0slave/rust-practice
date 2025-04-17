use std::env; // Import the env module
use std::fs::File; // Import File from std::fs
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_count = std::collections::HashMap::new();
    let mut pairs = 0;

    for &sock in ar {
        // Increment the count of the current sock color
        let count = sock_count.entry(sock).or_insert(0);
        *count += 1;
    }

    // Count the pairs
    for &count in sock_count.values() {
        pairs += count / 2;  // Each pair is 2 socks
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
