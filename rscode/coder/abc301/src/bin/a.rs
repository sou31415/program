fn main() {
    proconio::input! {
        n:usize,s:String
    }
    println!(
        "{}",
        if s.matches("T").count() > s.matches("A").count() {
            'T'
        } else if s.matches("T").count() < s.matches("A").count() {
            'A'
        } else {
            ('T' as u8 + 'A' as u8 - s.chars().nth(n - 1).unwrap() as u8) as char
        }
    );
}
