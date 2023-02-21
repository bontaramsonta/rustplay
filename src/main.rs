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
    let result = longest_common_prefix(v);
    println!("----------{_a} {result:#?}");
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut current_lcp = strs[0].clone();
    for x in &strs[1..] {
        loop {
            if current_lcp.is_empty() {
                return current_lcp;
            } else if x.starts_with(&current_lcp) {
                break;
            } else {
                current_lcp = current_lcp[..current_lcp.len() - 1].to_string();
            }
        }
    }
    current_lcp
}
