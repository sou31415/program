//submittion passed
use proconio::input;
fn main() {
    input! {
        a : String,
    }
    println!("{}", a.len() - a.matches("00").count());
}
