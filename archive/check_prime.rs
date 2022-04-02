use std::io::{self,Write};

fn ask<T: std::str::FromStr>(question: &str) -> T where <T as std::str::FromStr>::Err: std::fmt::Debug{
  let mut input = String::new();
  print!("{question}");
  io::stdout().flush().unwrap();
  io::stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}

fn check_prime(& mut n: &mut usize) -> bool {
  if n<2 {
    false
  }
  else if n==2 || n==3 {
    true
  }
  else if n%2==0 || n%3==0 {
    println!("div by 2 || 3");
    false
  }
  else {
    println!("loop");
    let mut c: u32=0;
    let mut i:usize = 5;
    loop {
      println!("{} iter {} {}",c,i,i+2);
      if i*i>n {
        break true;
      }
      if n%i==0 || n%(i+2)==0 {
        break false;
      }
      i+=6;
      c+=1;
    }
  }
}

fn main(){
  let mut n = ask::<usize>("Enter a number:");
  let f = check_prime(& mut n);
  println!("{f}")
}