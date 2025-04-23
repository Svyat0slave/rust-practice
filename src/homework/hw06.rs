fn print_tree(triangles: usize) {
    let width = 2 * triangles - 1;

    (1..=triangles)
        .flat_map(|height| (1..=height))
        .map(|row| {
            let stars = "*".repeat(2 * row - 1);
            let spaces = " ".repeat((width - stars.len()) / 2);
            format!("{}{}", spaces, stars)
        })
        .for_each(|line| println!("{}", line));
}

fn main() {
    let triangles = 6;
    print_tree(triangles);
}
