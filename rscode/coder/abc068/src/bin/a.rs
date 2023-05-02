use proconio::input;
fn main() {
    input! {
      n:usize,
    }
    let mx: u32 = (0..=n).map(|x| x.trailing_zeros()).max().unwrap() as u32;
    println!("{}", 2_usize.pow(mx as u32));
}
