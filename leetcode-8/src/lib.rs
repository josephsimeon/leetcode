fn my_atoi(s: String) -> i32 {
    s.parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
}
