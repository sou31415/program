use proconio::input;
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        d:i32,
        e:i32,
        f:i32,
        mut  h:i32,
    }
    let mut h2: i32 = h;
    let mut td: i32 = 0;
    let mut ad: i32 = 0;
    judge(a, b, c, &mut h, &mut td);
    judge(d, e, f, &mut h2, &mut ad);
    output(td, ad);
}
fn judge(x: i32, y: i32, z: i32, j: &mut i32, result: &mut i32) {
    while *j >= (x + z) {
        *result += x * y;
        *j -= (x + z);
    }
    if *j > x {
        *result += x * y;
    } else if *j <= x {
        *result += *j * y;
    }
}

fn output(q: i32, r: i32) {
    if q > r {
        println!("Takahashi");
    } else if q < r {
        println!("Aoki");
    } else if q == r {
        println!("Draw");
    }
}
