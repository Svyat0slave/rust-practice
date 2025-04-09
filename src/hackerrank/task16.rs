use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();

    // Fill the hash map to count occurrences of each bird type
    for &bird in arr {
        *frequency_map.entry(bird).or_insert(0) += 1;
    }

    // Find the maximum frequency
    let max_frequency = frequency_map.values().cloned().max().unwrap();

    // Collect all bird types with the maximum frequency
    let mut max_birds = frequency_map
        .into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .map(|(bird_type, _)| bird_type)
        .collect::<Vec<i32>>();

    // Return the smallest type among those with the maximum frequency
    max_birds.sort();
    max_birds[0]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
