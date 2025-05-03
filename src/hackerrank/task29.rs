use std::io::{self, BufRead};

/*
 * Complete the 'viralAdvertising' function below.
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */
fn viralAdvertising(n: i32) -> i32 {
    let mut shared = 5;
    let mut cumulative_likes = 0;

    for _ in 0..n {
        let liked = shared / 2;
        cumulative_likes += liked;
        shared = liked * 3;
    }

    cumulative_likes
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let result = viralAdvertising(n);
    println!("{}", result);
}
