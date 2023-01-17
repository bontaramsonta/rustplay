// cargo watch -cqs 'cargo -q run < in'
use r::get_number;
fn main() {
    let n = get_number::<i32>().unwrap();
    println!("{n}!");
}
