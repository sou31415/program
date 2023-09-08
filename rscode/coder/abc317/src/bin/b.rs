use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n:usize,
        mut a:[usize;n]
    }
    a.sort();
    let set = a.iter().map(|&x| x).collect::<HashSet<usize>>();
    for i in a[0]..=a[n - 1] {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", a[n - 1] + 1);
}
