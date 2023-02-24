#[allow(unused_imports)]
use proconio::{input, marker::Chars, marker::Usize1};
fn main() {
    input! {
        h:usize, w:usize,
        a:[Chars;h],
    }
    let mut ab: (isize, isize) = (0, 0);
    let mut count: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' && count == 0 {
                ab = (i as isize, j as isize);
                count += 1;
            } else if a[i][j] == 'o' && count == 1 {
                ab.0 = ((i as isize) - ab.0).abs();
                ab.1 = ((j as isize) - ab.1).abs();
            } else {
                continue;
            }
        }
    }
    println!("{}", ab.0 + ab.1);
}
