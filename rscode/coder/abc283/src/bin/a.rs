//submit passed
use proconio::input;
fn main() {
    input! {
        a:usize , b:usize,
    }
    println!("{}", a.pow(b.try_into().unwrap()));
}
