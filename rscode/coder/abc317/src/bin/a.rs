use proconio::input;
fn main() {
    input! {
        n:usize,h:usize,x:usize,
        a:[usize;n]
    }
    for i in 0..n {
        if a[i] + h >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
