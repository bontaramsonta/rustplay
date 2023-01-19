use std::io;
use std::str::FromStr;

pub fn get_input<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<T>()
}

pub fn get_space_seperated<T: FromStr>() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().split_ascii_whitespace().map(|str| str.parse::<i32>().expect("failed to parse")).collect()
}