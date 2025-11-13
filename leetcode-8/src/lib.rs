use std::num::IntErrorKind;

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

    match s.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow => i32::MAX,
                IntErrorKind::NegOverflow => i32::MIN,
                IntErrorKind::Empty => 0,
                _ => 0,
            }
        },
    }
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

    #[test]
    fn test_words_and_987() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test_neg91283472332() {
        assert_eq!(my_atoi("-91283472332".to_string()), i32::MIN);
    }

    #[test]
    fn test_21474836460() {
        assert_eq!(my_atoi("21474836460".to_string()), i32::MAX);
    }
}
