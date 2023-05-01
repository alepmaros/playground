// Given five positive integers, find the minimum and maximum values that can be calculated by summing exactly four of the five integers.
// Then print the respective minimum and maximum values as a single line of two space-separated long integers.

use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut sorted_arr: Vec<i64> = arr.to_vec().iter().map(|&x| x as i64).collect();
    sorted_arr.sort();
    
    let mini: i64 = sorted_arr.iter().take(4).copied().sum();
    let maxi: i64 = sorted_arr.iter().rev().take(4).copied().sum();
    
    println!("{} {}", mini, maxi);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
