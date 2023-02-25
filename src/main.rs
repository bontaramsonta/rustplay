use std::cmp::{max, min};

// cargo watch -w src/main.rs -w in -cqs 'cargo -q run < in > out'
use rust_play::*;

fn main() {
    let test_cases = get_input::<usize>().unwrap();
    for i in 0..test_cases {
        solve(i);
    }
}

fn solve(_a: usize) {
    println!("CASE: {_a}");
    // let n = get_input::<String>().unwrap();
    let v = get_space_separated::<i32>();
    let result = max_profit(v);
    println!("---------- {result:#?}");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let forward_iter = prices.iter();
    let reverse_iter = prices.iter().rev();
    let (minima, maxima) = forward_iter
        .zip(reverse_iter)
        .enumerate()
        .fold(0, |acc, (i, forward_ele, rev_ele)| {})
        .unwrap();
    maxima - minima
}
