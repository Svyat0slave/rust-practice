fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let shift = ((n % len) + len) % len;

    let (left, right) = s.split_at((len - shift) as usize);
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (n, expected) in shifts.iter() {
        let result = rotate(s.clone(), *n);
        println!("rotate({}, {}): {}", s, n, result);
        assert_eq!(result, *expected);
    }

    println!("All tests passed!");
}
