use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
      n:usize,mut a:[usize;n],

    }
    let mut ans: usize = 0;
    let mut s = a.clone();
    s.sort();
    s.dedup();
    if s.len() == 1 {
        println!("0");
        return;
    }
    let q = s[0];
    for i in 1..n {
        if a[i] % q != 0 {
            println!("-1");
            return;
        } else {
            if q == a[i] {
                continue;
            }
            a[i] /= q;
            while a[i] >= 2 {
                if a[i] / 2 >= q && a[i] % 2 == 0 {
                    a[i] /= 2;
                    ans += 1;
                }
                if a[i] == 1 {
                    break;
                }
                if a[i] % 3 == 0 && a[i] / 3 >= q {
                    a[i] /= 3;
                    ans += 1;
                }
                if a[i] == 1 {
                    break;
                }
                if a[i] % 2 != 0 && a[i] % 3 != 0 {
                    println!("-1");
                    return;
                }
            }
        }
    }
    println!("{}", ans);
}
