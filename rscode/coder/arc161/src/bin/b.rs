use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        t:usize,a:[usize;t],
    }
    let mut v: Vec<usize> = vec![];
    for i in (0..64).combinations(3) {
        let mut c: usize = 0;
        for j in 0..3 {
            c += (1 << i[j]);
        }
        v.push(c);
    }
    v.sort();
    for i in 0..t {
        if a[i] < 7 {
            println!("-1");
            continue;
        }
        if let Ok(_) = v.binary_search(&a[i]) {
            println!("{}", a[i]);
        } else if let Err(x) = v.binary_search(&a[i]) {
            println!("{}", v[x - 1]);
        }
    }
}
