pub fn is_palindrome(s: String) -> bool {
    let s_iter = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    let s_rev_iter = s_iter.clone().rev();
    s_iter.zip(s_rev_iter).all(|(a, b)| a == b)
}
fn main() {
    let s = String::from("A man, a plan, a canal: Panama");
    let result = is_palindrome(s);
    println!("result = {}", result);
}
