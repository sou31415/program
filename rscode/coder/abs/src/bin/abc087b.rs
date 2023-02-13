use proconio::input;
fn main() {
    input! {
        a:usize,b:usize,c:usize,x:usize,
    }
    let mut result: usize = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if (500 * i + 100 * j + 50 * k) == x {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
