fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            }
        })
        .collect()
}

fn main() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    for (a, b) in data.iter() {
        let result = invert_the_case(a.to_string());
        println!("Input: {}, Output: {}", a, result);
        println!("Expected Output: {}", b);
        assert_eq!(result, *b);

        let result_back = invert_the_case(b.to_string());
        println!("Input: {}, Output: {}", b, result_back);
        println!("Expected Output: {}", a);
        assert_eq!(result_back, *a);
    }

    println!("All tests passed!");
}
