#![feature(iter_intersperse)]
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
    // let str = get_input::<String>().unwrap();
    let v = get_space_seperated::<i32>();
    let n = v.len();
    let result = shuffle(v, n);
    println!("----------{_a} {result:#?}");
}

fn shuffle(nums: Vec<i32>, n: usize) -> Vec<i32> {
    (0..n)
        .map(|i| nums[i / 2 + (i % 2) * n / 2])
        .collect::<Vec<_>>()
}
