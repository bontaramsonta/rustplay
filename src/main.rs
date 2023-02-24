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
    let a = get_input::<String>().unwrap();
    let b = get_input::<String>().unwrap();
    // let v = get_space_separated::<i32>();
    let result = add_binary(a, b);
    println!("---------- {result:#?}");
}

pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut _carry: char = '0';
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();
    let mut count = max(a.len(), b.len());
    while count > 0 {
        let a_bit = a_iter.next().unwrap_or('0');
        let b_bit = b_iter.next().unwrap_or('0');
        println!("{a_bit} {b_bit} {_carry}");
        result.push(match (a_bit, b_bit, _carry) {
            ('0', '0', '0') => '0',
            ('0', '0', '1') => {
                _carry = '0';
                '1'
            }
            ('1', '0', '0') | ('0', '1', '0') | ('1', '1', '1') => '1',
            ('1', '0', '1') | ('0', '1', '1') | ('1', '1', '0') => {
                _carry = '1';
                '0'
            }
            _ => {
                panic!("invalid digit {a_bit} {b_bit} {_carry}");
            }
        });
        count -= 1;
    }
    if _carry == '1' {
        result.push('1');
    }
    result.chars().rev().collect::<String>()
}
