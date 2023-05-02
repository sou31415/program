use proconio::input;
fn main() {
    input! {
      n:u32,
    }
    let v: Vec<u32> = (1..=n).collect();
    let mx: u32 = v.iter().map(|x| x.trailing_zeros()).max().unwrap();
    println!("{}", 2_usize.pow(mx as u32));
}
