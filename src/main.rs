use std::cmp::max;

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
    let mut minimum = prices[0];
    let mut maximum_profit = 0;
    for price in prices {
        if price < minimum {
            minimum = price;
        }
        maximum_profit = max(maximum_profit, price - minimum);
    }
    maximum_profit
}
