
extern crate num;
use num::BigUint;
use num::FromPrimitive;
use num::One;

use std::io::{self, BufRead};

fn extraLongFactorials(n: u32) {
    let mut result = BigUint::one();

    for i in 2..=n {
        result *= BigUint::from_u32(i).unwrap();
    }

    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    extraLongFactorials(n);
}
