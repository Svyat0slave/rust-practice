use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

fn pageCount(n: i32, p: i32) -> i32 {
    // Calculate the number of page turns starting from the front
    let from_front = p / 2;

    // Calculate the number of page turns starting from the back
    let from_back = (n / 2) - (p / 2);

    // Return the minimum of the two
    if from_front < from_back {
        from_front
    } else {
        from_back
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Create file to write output to
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of pages
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the target page
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the pageCount function to calculate the result
    let result = pageCount(n, p);

    // Write the result to the output file
    writeln!(&mut fptr, "{}", result).ok();
}
