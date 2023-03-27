//submit passed
use proconio::input;

fn main() {
    input! {
        n:u64,
    }
    for _ in 0..n {
        input! {
            n:usize,
            a:[usize;n],
        }
        println!("{}", a.iter().filter(|x| **x % 2 == 1).count());
    }
}
