use std::io::{self, BufRead};

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut unique_ranked: Vec<i32> = Vec::new();
    for &score in ranked {
        if unique_ranked.last() != Some(&score) {
            unique_ranked.push(score);
        }
    }

    let mut result = Vec::new();
    let mut index = unique_ranked.len() as i32 - 1;

    for &score in player {
        while index >= 0 && score >= unique_ranked[index as usize] {
            index -= 1;
        }
        result.push(index + 2);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _ranked_count = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let ranked: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let _player_count = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let player: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = climbingLeaderboard(&ranked, &player);
    for rank in result {
        println!("{}", rank);
    }
}
