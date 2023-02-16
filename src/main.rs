// cargo watch -w src/main.rs -w in -cqs 'cargo -q run < in > out'
use rust_play::*;
use std::collections::HashMap;

fn main() {
    let test_cases = get_input::<usize>().unwrap();
    for i in 0..test_cases {
        solve(i);
    }
}

fn solve(_a: usize) {
    println!("CASE: {_a}");
    // let str = get_input::<String>().unwrap();
    let v = get_space_separated::<i32>();
    let result = num_identical_pairs(v);
    println!("----------{_a} {result:#?}");
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();
    for num in nums {
        let count = count_map.entry(num).or_insert(0);
        *count += 1;
    }
    count_map
        .values()
        .fold(0, |acc, value| acc + ((value * (value - 1)) / 2))
}
