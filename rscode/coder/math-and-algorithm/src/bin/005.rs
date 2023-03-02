use proconio::input;
fn main() {
    input! {
        a:usize , n:[usize;a],
    }
    let mut total: usize = 0;
    for i in n {
        total += i;
    }
    println!("{}", total % 100);
}
