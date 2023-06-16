use proconio::input;

fn main() {
    input! {
        mut a : usize,
    }
    for _ in 0..a {
        input! {
            mut b : usize,
        }
        let mut r: [usize; 2] = [0, 0];
        let mut i: usize = 2;
        while b > i * i * i {
            if b % i == 0 {
                break;
            }
            i += 1;
        }
        if b % (i * i) == 0 {
            r[0] = i;
            r[1] = b / (i * i);
        } else {
            r[1] = i;
            r[0] = ((b / i) as f64).sqrt() as usize;
        }
        println!("{} {}", r[0], r[1]);
    }
}
