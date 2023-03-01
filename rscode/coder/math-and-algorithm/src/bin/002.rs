use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize , b:usize , c:usize,
    }
    println!("{}", a + b + c);
}
