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
    let v = get_space_separated::<String>();
    let result = num_jewels_in_stones(v[0].to_owned(), v[1].to_owned());
    println!("---------- {result:#?}");
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .fold(0, |acc, stone| match jewels.contains(stone) {
            true => acc + 1,
            false => acc,
        })
}
