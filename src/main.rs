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
    let str = get_input::<String>().unwrap();
    // let v = get_space_separated::<String>();
    let result = is_valid(str);
    println!("----------{_a} {result:#?}");
}

pub fn is_valid(s: String) -> bool {
    let mut bracket_stack: Vec<char> = Vec::new();
    for bracket in s.chars() {
        match bracket {
            '}' | ')' | ']' => {
                if let Some(last_char) = bracket_stack.pop() {
                    if !((last_char, bracket) == ('{', '}')
                        || (last_char, bracket) == ('(', ')')
                        || (last_char, bracket) == ('[', ']'))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                bracket_stack.push(bracket);
            }
        }
    }
    bracket_stack.is_empty()
}
