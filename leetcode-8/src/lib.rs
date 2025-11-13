fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    let mut end = s.len();

    let mut s_chars_iter = s.chars().peekable();
    let mut skip = 0;
    if let Some(next) = s_chars_iter.peek() {
        if *next == '-' || *next == '+' {
            skip = 1;
        }
    }

    'search: for (i, ch) in s_chars_iter.skip(skip).enumerate() {
        if !ch.is_digit(10) {
            end = i + skip;
            break 'search;
        }
    }

    let s = &s[..end];
    println!("{s}");

    s.parse::<i32>().unwrap()
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

    #[test]
    fn test_1337c0d3() {
        assert_eq!(my_atoi("1337cd0d3".to_string()), 1337);
    }
}
