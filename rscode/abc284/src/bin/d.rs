//submit passed
use proconio::input;

fn main() {
    input! {
        mut a : usize,
    }
    for _ in 0..a {
        input! {
            mut b : usize,
        }
        let mut result: [usize; 2] = [0, 0];
        let mut i: usize = 2;
        while b > i * i * i {
            if b % i == 0 {
                break;
            }
            i += 1;
        }
        if b % (i * i) == 0 {
            result[0] = i;
            result[1] = b / (i * i);
        } else {
            result[1] = i;
            result[0] = ((b / i) as f64).sqrt() as usize;
        }
        println!("{} {}", result[0], result[1]);
    }
}
