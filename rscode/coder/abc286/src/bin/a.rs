use proconio::input;
fn main() {
    input! {
        n:usize,
        mut p:usize,
        mut q:usize,
        mut r:usize,
        mut s:usize,
        mut a:[usize;n],
    }
    let mut decoy = 0;
    for i in 0..=(q - p) {
        decoy = a[p - 1 + i];
        a[p - 1 + i] = a[r - 1 + i];
        a[r - 1 + i] = decoy;
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
}
