use proconio::input;
fn main() {
    input! {
        n:usize,a:[String;n],
    }
    let mut v: Vec<bool> = vec![false; n];
    let mut q = a.clone();
    q.sort();
    q.dedup();
    for i in 0..n {
        let k = q.binary_search(&a[i]).unwrap();
        if !v[k] {
            v[k] = true;
            println!("{}", i + 1);
        }
    }
}
