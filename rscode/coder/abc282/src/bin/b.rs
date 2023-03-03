use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize ,
        a:[Chars;n],
    }
    let mut jd: bool = false;
    let mut result: usize = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            jd = false;
            for k in 0..m {
                if a[i][k] != 'o' && a[j][k] != 'o' {
                    jd = true;
                    break;
                }
            }
            if !jd {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
