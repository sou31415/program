use proconio::input;

fn main() {
    input! {
        a:usize,
    }
    println!(
        "{}",
        if a == 3 || a == 5 || a == 7 {
            "YES"
        } else {
            "NO"
        }
    );
}
