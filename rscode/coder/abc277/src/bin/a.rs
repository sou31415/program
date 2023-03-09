use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize , x:usize,
        a:[usize;n],
    }
    for i in 0..n {
        if a[i] == x {
            println!("{}", i + 1);
            break;
        }
    }
}
