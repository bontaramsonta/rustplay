# Rust Archive
cargo-watch -cqx run
### 1. understanding the data ownership and borrowing
```rust
fn return_length(s: &String) -> usize {  // s got ownership of "hello world"
  s.len()
}

fn main(){
  let mut s = String::from("Hello");  // s comes into scope
  s.push_str("_world");                // s is mutated
  let length = return_length(&s);  // s is "moved" and also "shadow declared"/given back the ownership
  println!("{} has {} characters",s,length);
}
```
### 2. custom string iteration function
```rust
fn iterate_string(s: &String,callback: fn(char,usize) ) { 
  let bytes = s.as_bytes();
  for (i,item) in bytes.iter().enumerate() {callback(*item as char,i);}
}

let s = String::from("Hello");
iterate_string(&s, |item,i|{
  println!("{} {}",item,i);
  }
);
```

### 3. enums and match
```rust
#[derive(Debug)]
enum Difficulty {
  Low(String),
  Medium(String),
  High(String)
}
fn set_difficulty(diff:Difficulty) -> String {
  match diff {
    Difficulty::Low(i) => {i}
    Difficulty::Medium(i) => {i}
    Difficulty::High(i) => {i}
  }
}
println!("{:?}",set_difficulty(Difficulty::Low(String::from("You think you're afraid..huh?"))));
```

### 4. space seperated input
takes
```rust
fn space_sep(mut max_number: usize) -> Vec<i32> {
  let mut input = String::new();
  io::stdin().read_line(& mut input).expect("invalid input");
  input
  .trim()
  .split(" ")
  .filter_map(|x| {
    match x.parse::<i32>() {
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
  let n = ask::<usize>("Enter a no: ");
  let a = space_sep(n as i32);
  println!("{a:?}");
}
```

### 5. number parser (error handled)
```rust
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
  let x = ask("Enter 1st number:");
  let y = ask("Enter 2nd number:");
  println!("{x} {y}");
}
```

### 6. number parser (ttp)
```rust
use std::{
  io::stdin,
  str::FromStr,
  fmt::Debug
};
fn ask<T>() -> T where T: FromStr, <T as FromStr>::Err:Debug{
  let mut input = String::new();
  io::stdin().read_line(& mut input).expect("invalid input");
  input.trim().parse::<T>().unwrap()
}
let n = ask::<i32>();
```