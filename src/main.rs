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
    // let nums = get_space_separated::<i32>();
    let result = climb_stairs(n);
    println!("---------- {result:#?}");
}

pub fn climb_stairs(n: i32) -> i32 {
    let (mut a, mut b) = (1, 1);
    for _ in 1..n {
        let sum = a + b;
        b = a;
        a = sum;
    }
    a
}
