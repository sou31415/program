pub fn abc239_b(x: i128) -> i128 {
    if x > 0 {
        (x - (x % 10)) / 10
    } else if x < 0 && x % 10 == 0 {
        x / 10
    } else {
        (x - (10 - x % 10)) / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let case1 = abc239_b(47_i128);
        let case2 = abc239_b(-24);
        let case3 = abc239_b(50);
        let case4 = abc239_b(-30);
        let case5 = abc239_b(987654321987654321);

        assert_eq!(case1, 4);
        assert_eq!(case2, -3);
        assert_eq!(case3, 5);
        assert_eq!(case4, -3);
        assert_eq!(case5, 98765432198765432);
    }
}
