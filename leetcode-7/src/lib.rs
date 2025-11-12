fn reverse(x: i32) -> i32 {
    if x == 0 { return 0 };

    let sign = x.abs() / x;

    let s_rev = x.abs().to_string().chars().rev().collect::<String>();

    match s_rev.parse::<i32>() {
        Ok(num) => num * sign,
        Err(_) => 0,
    }
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

    #[test]
    fn test_0() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn test_1534236469() {
        assert_eq!(reverse(1534236469), 0);
    }
}
