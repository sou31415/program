use proconio::{fastout, input};
use regex::Regex;

#[fastout]
fn main() {
    input! {
        s:String,
    }
    let r = Regex::new(r"^[A-Z][1-9][0-9]{5}[A-Z]$").unwrap();
    if r.is_match(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
