use std::io;
use std::str::FromStr;

pub fn get_number<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse()
}
