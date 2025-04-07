const WIDTH: usize = 29;
const HEIGHT: usize = 15;

fn main() {
    let mut envelope = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {
                envelope.push('*'); // Верхня та нижня межі
            } else if j == 0 || j == WIDTH - 1 {
                envelope.push('*'); // Ліва та права межі
            } else if j == (i * WIDTH / HEIGHT) || j == (WIDTH - 1 - (i * WIDTH / HEIGHT)) {
                envelope.push('*'); // Діагоналі з урахуванням пропорційності
            } else {
                envelope.push(' ');
            }
        }
        envelope.push('\n'); // Перехід на новий рядок
    }

    println!("{}", envelope);
}