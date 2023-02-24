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
    // let n = get_input::<usize>().unwrap();
    let v = get_space_separated::<i32>();
    let result = plus_one(v);
    println!("---------- {result:#?}");
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry: i32 = 1;
    let mut digits = digits
        .iter()
        .rev()
        .map(|x| {
            let temp = *x + carry;
            carry = temp / 10;
            temp % 10
        })
        .collect::<Vec<i32>>();
    if carry != 0 {
        digits.push(carry);
    }
    digits.reverse();
    digits
}
