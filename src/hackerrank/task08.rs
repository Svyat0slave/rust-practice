use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8); // Split the input: "07:05:45PM" -> ("07:05:45", "PM")
    let mut parts = time
        .split(':')
        .map(|p| p.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if period == "PM" && parts[0] != 12 {
        parts[0] += 12;
    } else if period == "AM" && parts[0] == 12 {
        parts[0] = 0;
    }

    format!("{:02}:{:02}:{:02}", parts[0], parts[1], parts[2])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
