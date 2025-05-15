use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'appendAndDelete' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. STRING t
 *  3. INTEGER k
 */

fn appendAndDelete(s: &str, t: &str, k: i32) -> String {
    let s_len = s.len();
    let t_len = t.len();

    // Find common prefix length
    let mut common_length = 0;
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let min_len = std::cmp::min(s_len, t_len);

    for i in 0..min_len {
        if s_bytes[i] == t_bytes[i] {
            common_length += 1;
        } else {
            break;
        }
    }

    // Calculate minimum operations required to convert s to t
    let min_ops = (s_len - common_length) + (t_len - common_length);

    // Convert min_ops (usize) to i32 safely
    let min_ops_i32 = min_ops as i32;

    if min_ops_i32 > k {
        return "No".to_string();
    } else if (k - min_ops_i32) % 2 == 0 {
        // Extra operations can be balanced by deleting and appending dummy characters
        return "Yes".to_string();
    } else if k >= (s_len + t_len) as i32 {
        // If k is large enough to delete entire s and build t from empty string
        return "Yes".to_string();
    } else {
        return "No".to_string();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();
    let t = stdin_iterator.next().unwrap().unwrap();
    let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = appendAndDelete(&s, &t, k);

    writeln!(&mut fptr, "{}", result).ok();
}
