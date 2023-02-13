use proconio::input;
fn main() {
    input! {
        a:String,
    }
    println!("{}", a.matches("1").count());
}
