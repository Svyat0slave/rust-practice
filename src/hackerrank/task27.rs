use std::io;

fn angryProfessor(k: i32, a: &[i32]) -> String {
    let on_time = a.iter().filter(|&&time| time <= 0).count() as i32;
    if on_time < k {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nk: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = nk[0];
        let k = nk[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = angryProfessor(k, &a);
        println!("{}", result);
    }
}
