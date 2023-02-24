use proconio::input;
fn main() {
    input! {
        x:i128,
    }
    if x > 0 {
        println!("{}", (x - (x % 10)) / 10);
    } else if x < 0 && x % 10 != 0 {
        println!("{}", (x - 10 + (x % 10)) / 10);
    } else {
        println!("{}", (x + (x % 10)) / 10);
    }
}
