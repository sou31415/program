use proconio::input;
fn main() {
    input! {
        n:usize,mut x:usize,
    }
    let k: usize = 1;
    let mut b: usize = n;
    let mut a: usize = 1;
    for i in 0..64 {
        if x == 0 {
            break;
        }
        if k << i & x != 0 {
            a *= b;
            b *= b;
            x -= (k << i);
        } else {
            b *= b;
        }
        if x == 0 {
            break;
        }
    }
    println!("{}", a);
}

