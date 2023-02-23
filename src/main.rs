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
    let n = get_input::<i32>().unwrap();
    let v = get_space_separated::<i32>();
    let result = search_insert(v, n);
    println!("----------{_a} {result:#?}");
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search_by_key(&target, |&num| num) {
        Ok(index) => index as i32,
        Err(index) => index as i32,
    }
}
