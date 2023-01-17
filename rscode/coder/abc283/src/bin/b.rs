//submit passed
use proconio::input;
fn main() {
    input! {
        a:usize,
        mut b:[usize;a],
        c:usize,
    }
    for _ in 0..c {
        input! {
            d:usize,
        }
        if d == 1 {
            input! {
                e:usize,
                d:usize,
            }
            b[e - 1] = d;
        } else {
            input! {
                f:usize,
            }
            println!("{}", b[f - 1]);
        }
    }
}
