use proconio::input;
fn main() {
    input! {
        a:[usize;5],
    }
    let mut cnt = 5;
    for i in 0..4 {
        for j in (i + 1)..=4 {
            if a[i] == a[j] {
                cnt -= 1;
            }
        }
    }
    println!("{}", cnt);
}
