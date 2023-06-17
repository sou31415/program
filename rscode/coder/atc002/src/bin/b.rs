use proconio::input;
fn main() {
    input! {
        n:usize,m:usize,mut p:usize,
    }
    println!("{}", powm(n, p, m));
}

pub fn powm(n: usize, c: usize, m: usize) -> usize {
    let k: usize = 1;
    let mut x = c;
    let mut b: usize = n;
    let mut a: usize = 1;
    let mut i = 0;
    while x != 0 {
        if k << i & x != 0 {
            a = (a * b) % m;
            b = (b * b) % m;
            x ^= (k << i);
        } else {
            b = (b * b) % m;
        }
        i += 1;
    }
    a
}
