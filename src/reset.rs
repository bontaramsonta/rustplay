use std::io::{self,Write};

fn ask(question: &str) -> i32{
  loop {
    let mut input = String::new();
    print!("{question}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(& mut input).expect("invalid input");
    match input.trim().parse::<i32>() {
      Ok(num) => {break num},
      Err(e) => {eprintln!("try again: {:?}",e.kind());continue;}
    }
  }
}


fn main(){
  let n = ask::<usize>("Enter a number:");
  
}