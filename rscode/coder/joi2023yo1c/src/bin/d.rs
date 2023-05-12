use proconio::input;
fn main() {
    input! {
      n:usize,a:[usize;n],
    }
    let mut q = a.clone();
    q.sort();
    for i in a.iter() {
        for j in 0..n {
            if q[j] == *i {
                println!("{}", j + 1);
                break;
            }
        }
    }
}
