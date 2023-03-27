fn main() {
    proconio::input! {
        n:usize,
        t:[String;n],
    }
    println!(
        "{}",
        if t.iter()
            .any(|i| i == "and" || i == "not" || i == "the" || i == "that" || i == "you")
        {
            "Yes"
        } else {
            "No"
        }
    );
}
