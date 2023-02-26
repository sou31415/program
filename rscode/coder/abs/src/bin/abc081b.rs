use proconio::input;
fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    println!("{}", a.iter().map(|x| x.trailing_zeros()).min().unwrap());
}
