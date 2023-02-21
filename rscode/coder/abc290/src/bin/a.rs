#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        a : [usize;n],
        b:[Usize1;m],
    }
    let mut result: usize = 0;
    for i in b {
        result += a[i];
    }
    println!("{}", result);
}
