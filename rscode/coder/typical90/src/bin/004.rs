use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h:usize,w:usize,
        ab:[[usize;w];h]
    }
    let mut v: Vec<usize> = vec![];
    let mut v2: Vec<usize> = vec![];
    for i in 0..h {
        let q: usize = ab[i].iter().sum();
        v.push(q);
    }
    for i in 0..w {
        let mut q: usize = 0;
        for j in 0..h {
            q += ab[j][i];
        }
        v2.push(q);
        q = 0;
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", v[i] + v2[j] - ab[i][j]);
        }
        println!("");
    }
}
