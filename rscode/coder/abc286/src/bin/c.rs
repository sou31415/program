use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize , a:i64 , b:i64 ,
        s:Chars,
    }
    let mut result = b * n as i64;
    for i in 0..n {
        let mut k = 0;
        for j in 0..n / 2 {
            if s[(i + j) % n] != s[(i + n - 1 - j) % n] {
                k += b;
            }
        }
        result = result.min(a * i as i64 + k)
    }
    println!("{}", result);
}
