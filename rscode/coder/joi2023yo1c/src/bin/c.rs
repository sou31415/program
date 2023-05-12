use proconio::input;
fn main() {
    input! {
      n:usize,a:[usize;n],
    }
    let mut q = a.clone();
    q.sort();
    for i in a.iter() {
        println!("{}", q.binary_search(&i).unwrap() + 1);
    }
}
