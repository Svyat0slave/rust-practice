use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn permutationEquation(p: &[i32]) -> Vec<i32> {
    let n = p.len();
    let mut pos = vec![0; n + 1];

    for (i, &val) in p.iter().enumerate() {
        pos[val as usize] = (i + 1) as i32;
    }

    let mut result = Vec::new();
    for x in 1..=n as i32 {
        let y = pos[pos[x as usize] as usize];
        result.push(y);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = permutationEquation(&p);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
