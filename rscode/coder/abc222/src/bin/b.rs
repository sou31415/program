use proconio::input;
fn main() {
    input! {
        n : usize,
        p : usize,
        a : [usize;n],
    }
    println!("{}", a.into_iter().filter(|x| x < &p).count());
}
