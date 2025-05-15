use std::io;

/*
 * Complete the 'libraryFine' function below.
 * The function returns an INTEGER:
 * - the amount of the fine or 0 if none.
 */
fn libraryFine(d1: i32, m1: i32, y1: i32, d2: i32, m2: i32, y2: i32) -> i32 {
    if y1 < y2 {
        0
    } else if y1 == y2 && m1 < m2 {
        0
    } else if y1 == y2 && m1 == m2 && d1 <= d2 {
        0
    } else if y1 == y2 && m1 == m2 {
        15 * (d1 - d2)
    } else if y1 == y2 {
        500 * (m1 - m2)
    } else {
        10000
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let returned: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let due: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let fine = libraryFine(returned[0], returned[1], returned[2], due[0], due[1], due[2]);

    println!("{}", fine);
}
