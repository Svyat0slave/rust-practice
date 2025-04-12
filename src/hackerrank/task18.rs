use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Calculate the total cost of the items Anna and Brian share
    let total_cost: i32 = bill.iter().sum();

    // Subtract the cost of the item Anna didn't eat
    let anna_share = (total_cost - bill[k as usize]) / 2;

    // Compare the amount Anna paid with her actual share
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);  // Refund amount
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the first line of input
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();  // Total items (not used)
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();  // Item Anna didn't eat

    // Read the second line of input (bill amounts)
    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Read the third line of input (amount Brian charged Anna)
    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the function with the parsed data
    bonAppetit(&bill, k, b);
}
