use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f64;
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for &num in arr {
        if num > 0 {
            positive += 1;
        } else if num < 0 {
            negative += 1;
        } else {
            zero += 1;
        }
    }

    println!("{:.6}", positive as f64 / n);
    println!("{:.6}", negative as f64 / n);
    println!("{:.6}", zero as f64 / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
