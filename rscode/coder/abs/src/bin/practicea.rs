use proconio::input;
fn main() {
    input! {
        a:usize,b:usize,c:usize,d:String,
    }
    println!("{} {}", a + b + c, d);
}
