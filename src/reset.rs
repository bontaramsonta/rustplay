use std::{
  io::{self,Write},
  str::FromStr,
  fmt::Debug
};
fn ask<T>(question: &str) -> T where T: FromStr, <T as FromStr>::Err:Debug{
  loop {
    let mut input = String::new();
    print!("{question}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(& mut input).expect("invalid input");
    match input.trim().parse::<T>() {
      Ok(num) => {break num},
      Err(e) => {println!("try again: {:?}",e);continue;}
    }
  }
}


fn main(){
  let n = ask::<usize>("Enter a number:");
  println!("{n}");
}