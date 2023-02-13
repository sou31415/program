use proconio::input;
fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let mut alice: usize = 0;
    let mut bob: usize = 0;
    for i in 0..a.len() {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }
    println!("{}", alice - bob);
}
