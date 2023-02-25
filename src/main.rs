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
    // let n = get_input::<String>().unwrap();
    let v = get_space_separated::<i32>();
    let result = max_profit(v);
    println!("---------- {result:#?}");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2).fold(0, |profit, prize_window| {
        let current_prize = prize_window.get(0).unwrap();
        let next_prize = prize_window.get(1).unwrap();
        match current_prize.cmp(&next_prize) {
            std::cmp::Ordering::Less => profit + (next_prize - current_prize),
            _ => profit,
        }
    })
}
