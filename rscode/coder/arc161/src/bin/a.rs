use proconio::input;

fn main() {
    input! {
        n:usize,mut a:[usize;n],
    }
    a.sort();
    let mut v: Vec<usize> = Vec::new();
    for i in 0..(n / 2) {
        let r = (n / 2) + 1 + i;
        if a[i] < a[r] {
            v.push(a[i]);
            v.push(a[r]);
        } else {
            println!("No");
            return;
        }
    }
    v.push(a[n / 2]);

    for i in 0..(n - 1) {
        if i % 2 == 0 {
            if v[i] >= v[i + 1] {
                println!("No");
                return;
            }
        } else {
            if v[i] <= v[i + 1] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
