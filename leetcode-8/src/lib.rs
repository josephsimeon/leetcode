fn my_atoi(s: String) -> i32 {
    s.trim_start().parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_spaced_neg42() {
        assert_eq!(my_atoi(" -042".to_string()), -42);
    }
}
