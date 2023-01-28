use proconio::input;
fn main() {
    input! {
        n:usize,
        a:[String;n],
    }
    let mut cnt: usize = 0;
    for i in a {
        if i == "For" {
            cnt += 1;
        }
    }
    println!("{}", if (cnt * 2) > n { "Yes" } else { "No" });
}
