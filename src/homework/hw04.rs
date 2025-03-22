const WIDTH: usize = 17; // Ширина ромба (непарне число)
const HEIGHT: usize = WIDTH; // Висота ромба

fn main() {
    let mut output = String::new(); // Зберігаємо результат у змінну

    // Верхня половина ромба
    for i in 0..(HEIGHT / 2 + 1) {
        output.push_str(&" ".repeat(HEIGHT / 2 - i));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    // Нижня половина ромба
    for i in (0..(HEIGHT / 2)).rev() {
        output.push_str(&" ".repeat(HEIGHT / 2 - i));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    print!("{}", output); // Вивід всього ромба одним викликом
}