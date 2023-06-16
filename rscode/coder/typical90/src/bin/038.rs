use proconio::input;
fn main() {
    input! {
      a:u128,b:u128,
    }
    let c = a * b;
    if (c / f(a, b)) > 100_000_000_000_000_0000 {
        println!("Large");
    } else {
        println!("{}", c / f(a, b));
    }
}

fn f(a: u128, b: u128) -> u128 {
    let mut c = a;
    let mut d = b;
    if d > c {
        std::mem::swap(&mut c, &mut d);
    }
    while c % d != 0 {
        let s = d;
        d = c % d;
        c = s;
    }
    d
}
