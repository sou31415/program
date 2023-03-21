use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize , b:usize , c:usize,
    }
    let jd: usize = c;
    if c < (b / a) {
        println!("{}", c);
    } else {
        println!("{}", b / a);
    }
}
