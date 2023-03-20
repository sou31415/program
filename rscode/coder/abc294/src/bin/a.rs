fn main() {
    proconio::input! {
        n:usize,
        a:[usize;n],
    }
    let v: Vec<usize> = a.into_iter().filter(|x| x % 2 == 0).collect();
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
}
