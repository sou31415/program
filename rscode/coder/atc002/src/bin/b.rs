use proconio::input;
fn main() {
    input! {
        n:usize,m:usize,mut p:usize,
    }
    let mut i: usize = 0;
    let k: usize = 1;
    let mut q: usize = n;
    let mut result: usize = 1;
    while p != 0 {
        if p & (k << i) != 0 {
            result = (result % m * q % m) % m;
            p -= k << i;
        }
        q = q % m * q % m;
        i += 1;
    }
    println!("{}", result);
}
