fn main() {
    proconio::input! {
        n:usize,
        a:[usize;n],
    }
    let mut cnt = 0;
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        if set.contains(&a[i]) {
            cnt += 1;
            set.remove(&a[i]);
        } else {
            set.insert(a[i]);
        }
    }
    println!("{}", cnt);
}
