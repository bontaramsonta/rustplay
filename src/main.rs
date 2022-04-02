use std::{
  io::{self,Write},
  str::FromStr,
  fmt::Debug
};
fn ask<T>() -> T where T: FromStr, <T as FromStr>::Err:Debug{
  loop {
    let mut input = String::new();
    // print!("{question}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(& mut input).expect("invalid input");
    match input.trim().parse::<T>() {
      Ok(num) => {break num},
      Err(e) => {println!("try again: {:?}",e);continue;}
    }
  }
}
fn space_sep(mut max_number: usize) -> Vec<u32> {
  let mut input = String::new();  
  io::stdin().read_line(& mut input).expect("invalid input");
  input
  .trim()
  .split(" ")
  .filter_map(|x| {
    match x.parse::<u32>() {
      Ok(num) => {
        if max_number > 0 {
          max_number-=1;
          Some(num)
        }
        else { None }
      },
      Err(_) => { None }
    }
  }) .collect()
}


fn main(){
  let mut n1 = ask::<u32>();
  while n1>0 {
    let n2 = space_sep(2);
    let ans = 
      if n2[0] != 0 {
        (n2[1]*2)+n2[0]+1
      } else {
        1
      };
    println!("{ans}");
    n1-=1;
  };
}