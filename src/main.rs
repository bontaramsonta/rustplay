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
    let s = get_input::<String>().unwrap();
    // let v = get_space_separated::<i32>();
    let result = length_of_last_word(s);
    println!("----------{_a} {result:#?}");
}

pub fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .rsplit_once(' ')
        .unwrap_or(("", &s.trim()))
        .1
        .len()
        .try_into()
        .unwrap()
}
