use proconio::input;
fn main() {
    input! {
      n:usize,a:[usize;n],
    }
    let mut s = a.clone();
    s.sort();
    s.dedup();
    let mut q = vec![0_usize; s.len()];
    for i in a {
        let d = s.binary_search(&i).unwrap();
        q[d] += 1;
    }
    println!("{}", q.iter().map(|&x| x / 2).sum::<usize>());
}
