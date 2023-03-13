use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
    }
    let mut b: Vec<Vec<usize>> = vec![vec![]; m];
    let mut v: Vec<bool> = vec![false; n];
    for i in 0..m {
        input! {
            c:usize , a:[Usize1;c],
        }
        b[i] = a;
    }

    let mut result: usize = 0;
    for i in 1..(1 << m) {
        for j in 0..=m {
            if i & (1 << j) > 0 {
                for k in 0..b[j].len() {
                    v[b[j][k]] = true;
                }
            }
        }
        if v.iter().all(|x| *x) {
            result += 1;
        }
        v = vec![false; n];
    }
    println!("{}", result);
}
