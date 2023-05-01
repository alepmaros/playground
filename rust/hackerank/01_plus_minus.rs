// Given an array of integers, calculate the ratios of its elements that are positive, negative, and zero.
// Print the decimal value of each fraction on a new line with  places after the decimal.

use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f32;
    let mut n_pos: f32 = 0.0;
    let mut n_neg: f32 = 0.0;
    let mut n_zer: f32 = 0.0;
    
    for nb in arr.iter() {
        if *nb < 0 {
            n_neg += 1.0;
        } else if *nb == 0 {
            n_zer += 1.0;
        } else {
            n_pos += 1.0;
        }
    }

    println!("{:.6}\n{:.6}\n{:.6}", n_pos / n, n_neg / n, n_zer / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
