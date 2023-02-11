use proconio::input;

fn main() {
    input! {
      n: usize,
    }
    println!("{:0>10b}", n);
}
