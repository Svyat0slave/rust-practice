const WIDTH: usize = 15;
const HEIGHT: usize = 15;

fn main() {
    let mut envelope = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {
                envelope.push('*');
            } else if j == 0 || j == WIDTH - 1 {
                envelope.push('*');
            } else if j == i || j == WIDTH - i - 1 {
                envelope.push('*');
            } else {
                envelope.push(' ');
            }
        }
        envelope.push('\n');
    }

    println!("{}", envelope);
}