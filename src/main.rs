use std::collections::HashMap;

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
    let dp: HashMap<i32, i32> = HashMap::new();
    let result = climb_stairs(n, &dp);
    println!("---------- {result:#?}");
}

pub fn climb_stairs(n: i32, dp: &HashMap<i32, i32>) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => {
            *dp.get(&(n - 1)).get_or_insert(&climb_stairs(n - 1, dp))
                + *dp.get(&(n - 2)).get_or_insert(&climb_stairs(n - 2, dp))
        }
    }
}
