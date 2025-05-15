use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn cutTheSticks(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut sticks = arr.to_vec();

    while !sticks.is_empty() {
        result.push(sticks.len() as i32);
        let min = *sticks.iter().min().unwrap();
        sticks = sticks
            .into_iter()
            .filter_map(|x| if x > min { Some(x - min) } else { None })
            .collect();
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = cutTheSticks(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
