use proconio::input;
fn main() {
    input! {
        n:usize,a:[usize;n],
    }
    for i in a {
        print!("{:0>8b} ", i);
    }
    println!("");
}
