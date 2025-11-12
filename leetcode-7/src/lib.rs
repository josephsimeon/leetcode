fn reverse(x: i32) -> i32 {
    let sign = x.abs() / x;

    let s_rev = x.abs().to_string().chars().rev().collect::<String>();

    s_rev.parse::<i32>().unwrap() * sign
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_neg123() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_120() {
        assert_eq!(reverse(120), 21);
    }
}
