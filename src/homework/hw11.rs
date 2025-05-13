use std::time::{SystemTime, UNIX_EPOCH};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);

    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    let mut seed = duration.as_secs() as i32;

    for _ in 0..n {
        seed = seed.wrapping_add(1234567);
        result.push((seed % 90 + 10) as i32);
    }

    result
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_idx = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = (i, i + 1);
        }
    }

    (min_sum, min_idx.0, min_idx.1)
}

fn print_data_with_indexes(data: &[i32], min_sum: i32, idx1: usize, idx2: usize) {
    let max_index_len = data.len().to_string().len();

    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:width$}.  ", i, width = max_index_len);
    }
    println!();

    print!("data:   ");
    for val in data {
        print!("{:2} ", val);
    }
    println!();

    print!("indexes:");
    for i in 0..data.len() {
        if i == idx1 {
            print!(" \\__ ");
        } else if i == idx2 {
            print!(" __/ ");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}

fn main() {
    let data = gen_random_vector(20);
    let (min_sum, idx1, idx2) = min_adjacent_sum(&data);
    print_data_with_indexes(&data, min_sum, idx1, idx2);
}
