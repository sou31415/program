use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize , u:usize,
    }
    let mut che = 0;
    for i in 0..=n {
        for j in 0..=n - i {
            if i * 10000 + j * 5000 + (n - i - j) * 1000 == u {
                println!("{} {} {}", i, j, n - i - j);
                che = 1;
                break;
            }
        }
        if che == 1 {
            break;
        }
    }
    if che == 0 {
        println!("-1 -1 -1");
    }
}
