use proconio::input;
fn main() {
    input! {
      n:usize,
    }
    let mut cnt: usize = 63;
    loop {
        if (1 << cnt) <= n {
            println!("{}", cnt);
            return;
        }
        cnt -= 1;
    }
}
