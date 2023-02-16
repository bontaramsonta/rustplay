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
    let v = get_space_separated::<String>();
    let result = final_value_after_operations(v);
    println!("----------{_a} {result:#?}");
}

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |acc, x| match x.as_str() {
        "++X" | "X++" => acc + 1,
        "--X" | "X--" => acc - 1,
        _ => acc,
    })
}
