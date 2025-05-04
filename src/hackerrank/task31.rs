use std::io::{self, BufRead};

fn circularArrayRotation(a: &[i32], k: i32, queries: &[i32]) -> Vec<i32> {
    let n = a.len();
    let k = (k as usize) % n;

    queries.iter()
        .map(|&q| {
            let index = (q as usize + n - k) % n;
            a[index]
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N, K, Q
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, k, q) = (parts[0], parts[1], parts[2]);

    // Read the array
    let array_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = array_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read queries
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        queries.push(line.parse::<i32>().unwrap());
    }

    // Get the result
    let result = circularArrayRotation(&a, k as i32, &queries);

    // Output
    for val in result {
        println!("{}", val);
    }
}
