use std::{
  io::{self,Write},
  cmp::Ordering,
  str::FromStr,
  fmt::Debug,
  f32::NEG_INFINITY
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

fn space_sep(mut max_number: usize) -> Vec<i32> {
  let mut input = String::new();
  let mut prev = NEG_INFINITY as i32;  // asc: saving previous values
  io::stdin().read_line(& mut input).expect("invalid input");
  input
  .trim()
  .split(" ")
  .filter_map(|x| {
    match x.parse::<i32>() {
      Ok(num) => {
        if max_number > 0 { // filter-out extra elements
          if prev > (num as i32) { return None; } // filter-out decreasing values
          prev = num;
          max_number-=1;
          Some(num)
        }
        else { None }
      },
      Err(_) => { None }
    }
  }) .collect()
}

fn bin_search(a:& [i32], n: usize, e: i32) -> Option<usize> {
  let mut start: i32 = 0;
  let mut end: i32 = (n as i32) - 1;
  let mut mid: usize;
  while start <= end {
    mid = ((start+end)/2) as usize;
    match e.cmp(&a[mid])  {
      Ordering::Equal => { return Some(mid); }
      Ordering::Greater => { start = mid as i32+1; }
      Ordering::Less => { end = mid as i32-1; }
    }
  }
  return None;
}

fn main(){
  let n = ask::<usize>("Enter size: ");
  println!("Enter a sorted array (non int & decreasing nums will be dropped)");
  let a = space_sep(n);
  println!("{a:?}");
  let element = ask("search for: ");
  match bin_search(a.as_slice(),n,element) {
    Some(num) => { println!("{element} is at index {num}"); }
    None => { println!("{element} not found"); }
  }
}