use std::fmt::Debug;
use std::io;
use std::str::FromStr;

pub fn get_input<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<T>()
}

pub fn get_space_separated<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().expect("Failed to parse"))
        .collect()
}
