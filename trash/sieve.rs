use std::io::{self,Write};

fn ask<T: std::str::FromStr>(question: &str) -> T where <T as std::str::FromStr>::Err: std::fmt::Debug{
  let mut input = String::new();
  print!("{question}");
  io::stdout().flush().unwrap();
  io::stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}

fn sieve(n: usize) {
  let mut sieve: Vec<bool> = vec![true;n-1];
  for i in 0..(num::integer::sqrt(n)-1) {
    // println!("{} {} {:?}",i,i+2,sieve[i]);
    if sieve[i] {
      let mut j = (i+2)*2;
      while j<=n {
        // println!(" {} {}",j-2,j);
        sieve[j-2] = false;
        j=j+(i+2);
      }
      // println!("{} {} {:?}",i,i+2,sieve);
    }
  }
  for (i,x) in sieve.iter().enumerate() {
    if *x {
      print!("{} ",i+2);
    }
  }
  println!();
  io::stdout().flush().unwrap();
}

fn main(){
  let n = ask::<usize>("Enter a number:");
  sieve(n);
}