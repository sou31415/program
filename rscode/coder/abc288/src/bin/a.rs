use proconio::input;
fn main() {
    input! {
        a:usize,
    }
    for _ in 0..a {
        input! {
            b:isize,
            c:isize,
        }
        println!("{}", b + c);
    }
}
