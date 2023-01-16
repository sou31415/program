use proconio::input;
fn main() {
    input! {
        a:usize,
        b:usize,
    }
    if a + b * 7 > 30 {
        println!(0);
    } else {
        println!(1);
    }
}
