use std::collections::HashSet;

fn main() {
    let mut count = 0;

    for m in 1..10 {
        for u in 0..10 {
            for x in 0..10 {
                for a in 1..10 {
                    if HashSet::from([m, u, x, a]).len() < 4 {
                        continue;
                    }

                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                    let result = muxa * a;

                    if result > 9999 {
                        continue;
                    }

                    let s = (result / 1000) % 10;
                    let l = (result / 100) % 10;
                    let o = (result / 10) % 10;
                    let n = result % 10;

                    let digits = vec![m, u, x, a, s, l, o, n];
                    let unique: HashSet<_> = digits.iter().cloned().collect();

                    if unique.len() == 8 {
                        count += 1;
                        println!(
                            "  {}{}{}{} \n×    {}\n------\n  {}{}{}{}\n",
                            m, u, x, a, a, s, l, o, n
                        );
                    }
                }
            }
        }
    }

    println!("Кількість рішень: {}", count);
}
