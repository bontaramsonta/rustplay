use std::{
  io::stdin,
  str::FromStr,
  fmt::Debug
};
fn ask<T>() -> T where T: FromStr, <T as FromStr>::Err:Debug{
  let mut input = String::new();
  stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}


fn main(){
  let n = ask::<i32>();
  println!("{n}");
}