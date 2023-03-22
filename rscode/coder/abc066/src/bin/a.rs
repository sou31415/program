use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a:[usize;3],
    }
    a.sort();
    println!("{}", a[0] + a[1]);
}
