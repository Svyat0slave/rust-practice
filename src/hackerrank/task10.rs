use std::io::{self, BufRead};

/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s
 *  2. INTEGER t
 *  3. INTEGER a
 *  4. INTEGER b
 *  5. INTEGER_ARRAY apples
 *  6. INTEGER_ARRAY oranges
 */

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_on_house = apples.iter().filter(|&&d| {
        let position = a + d;
        position >= s && position <= t
    }).count();

    let oranges_on_house = oranges.iter().filter(|&&d| {
        let position = b + d;
        position >= s && position <= t
    }).count();

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (s, t) = (first_multiple_input[0], first_multiple_input[1]);

    let second_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (a, b) = (second_multiple_input[0], second_multiple_input[1]);

    let third_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let _m = third_multiple_input[0];
    let _n = third_multiple_input[1];

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
