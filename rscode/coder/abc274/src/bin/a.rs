use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:f64 , b:f64,
    }
    println!("{:.3}", b / a);
}
