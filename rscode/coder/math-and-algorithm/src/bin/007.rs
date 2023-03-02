use proconio::input;

fn main() {
    input! {
        n:usize,x:usize,y:usize,
    }
    let mut count: usize = 0;
    for i in x..=n {
        if i % x == 0 || i % y == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
