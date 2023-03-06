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
    // let jewels = get_input::<String>().unwrap();
    // let stone = get_input::<String>().unwrap();
    let v = get_space_separated::<i32>();
    let result = smaller_numbers_than_current(v);
    println!("---------- {result:#?}");
}

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .map(|(i, number)| {
            nums.iter().enumerate().skip_while(|(j, _)| *j == i).fold(
                0,
                |acc, (_, other_number)| match number.cmp(&other_number) {
                    std::cmp::Ordering::Greater => acc + 1,
                    _ => acc,
                },
            )
        })
        .collect()
}
