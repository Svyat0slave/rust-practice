use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

// Helper function to determine if the year is a leap year in the Julian calendar
fn is_julian_leap(year: i32) -> bool {
    year % 4 == 0
}

// Helper function to determine if the year is a leap year in the Gregorian calendar
fn is_gregorian_leap(year: i32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

fn dayOfProgrammer(year: i32) -> String {
    let mut day_of_programmer = 256;
    let mut month = 9; // September

    let day_of_month: i32;

    // Case 1: Julian calendar (1700-1917)
    if year >= 1700 && year <= 1917 {
        // In Julian calendar, February has 29 days in leap years and 28 otherwise
        if is_julian_leap(year) {
            // Leap year
            day_of_month = 12;
        } else {
            // Non-leap year
            day_of_month = 13;
        }
    }
    // Case 2: Transition year (1918)
    else if year == 1918 {
        // In 1918, after January 31, the next day is February 14
        // So February has 15 days in total
        day_of_month = 26;
    }
    // Case 3: Gregorian calendar (1919-2700)
    else {
        // In Gregorian calendar, February has 29 days in leap years and 28 otherwise
        if is_gregorian_leap(year) {
            // Leap year
            day_of_month = 12;
        } else {
            // Non-leap year
            day_of_month = 13;
        }
    }

    // Return the formatted date
    format!("{:02}.{:02}.{}", day_of_month, month, year)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = dayOfProgrammer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
