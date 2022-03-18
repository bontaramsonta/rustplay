use std::io::{self,Write};
use num;
fn ask<T: std::str::FromStr>(question: &str) -> T where <T as std::str::FromStr>::Err: std::fmt::Debug{
  let mut input = String::new();
  print!("{question}");
  io::stdout().flush().unwrap();
  io::stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}

fn gcd(x: u8, y: u8) -> u8 {
  if y == 0 {
    x
  }else {
    gcd(y, x%y)
  }
}

fn main(){
  let x = ask::<u8>("Enter 1st number:");
  let y = ask::<u8>("Enter 2nd number:");
  let ans1 = num::integer::gcd(x,y);
  let ans2 = gcd(x,y);
  assert_eq!(ans1,ans2);
  println!("{ans1} {ans2}");
}