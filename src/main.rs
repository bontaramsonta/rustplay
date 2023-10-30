// cargo watch -w src/main.rs -w in.txt -cqs 'cargo -q run < in.txt > out.txt 2>&1'
use rust_play::*;

#[derive(Debug)]
struct Fibonacci {
    n: usize,
    dp: Vec<i64>,
}
impl Fibonacci {
    fn new(n: usize) -> Self {
        let mut dp = vec![-1; n];
        dp[0] = 0;
        dp[1] = 1;
        Self { n, dp }
    }
    fn get_fib(&mut self, n: usize) -> i64 {
        let n = n - 1;
        if n >= self.n {
            // resize dp
            self.dp.resize(n + 1, -1);
            self.n = n + 1;
        }
        if self.dp[n] != -1 {
            // got cached value
            return self.dp[n];
        }
        self.dp[n] = self.get_fib(n) + self.get_fib(n - 1);
        self.dp[n]
    }
}
fn main() {
    let test_cases = get_input::<usize>().unwrap();
    let mut fib = Fibonacci::new(5);
    for i in 0..test_cases {
        solve(i, &mut fib);
    }
}

fn solve(a: usize, fib: &mut Fibonacci) {
    println!("CASE: {}", a + 1);
    let n = get_input::<usize>().unwrap();
    let result = fib.get_fib(n);
    dbg!(n, result);
}
