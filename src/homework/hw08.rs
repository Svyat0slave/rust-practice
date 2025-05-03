fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }
    let limit = (*n as f64).sqrt() as u32;
    for i in 3..=limit {
        if *n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (n, expected_prime) in test_data.iter() {
        let result = is_prime(n);
        println!("Testing number: {}", n);
        println!("Expected: {}", expected_prime);
        println!("Result: {}", result);
        assert_eq!(result, *expected_prime);
        println!();
    }

    println!("All tests passed!");
}
