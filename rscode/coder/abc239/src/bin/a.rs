use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:f64,
    }
    println!("{}", (a * (12800000_f64 + a)).sqrt());
}
