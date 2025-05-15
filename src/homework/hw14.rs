fn gray(n: u8) -> Vec<String> {
    let count = 1 << n;
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let s = format!("{:0width$b}", i, width = n as usize);
        result.push(s);
    }
    result
}

fn main() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "10", "11"]),
        (3, vec!["000", "001", "010", "011",
                 "100", "101", "110", "111"]),
        (4, vec!["0000", "0001", "0010", "0011",
                 "0100", "0101", "0110", "0111",
                 "1000", "1001", "1010", "1011",
                 "1100", "1101", "1110", "1111"]),
    ];

    for (n, expected) in test_data.iter() {
        let result = gray(*n);
        if result == *expected {
            println!("Test passed for n = {}", n);
        } else {
            println!("Test failed for n = {}", n);
            println!("Expected: {:?}\nGot:      {:?}", expected, result);
        }
    }
}
