use std::io::{self,Write};
use num::integer;
fn ask<T: std::str::FromStr>(question: &str) -> T where <T as std::str::FromStr>::Err: std::fmt::Debug{
  let mut input = String::new();
  print!("{question}");
  io::stdout().flush().unwrap();
  io::stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}

fn fact(n: u8) -> usize {
  if n>20 {
    panic!("too large")
  }
  let mut c: usize = n as usize;
  let mut fact:usize = 1;
  while c>1 {
    fact = fact * c;
    c-=1;
  }
  fact
}

fn main(){
  let n = ask::<u8>("Enter a number:");
  let fact = fact(n);
  // this is it 👇
  let z5 = integer::div_floor(n,5);

  println!("{fact} {z5}")
}