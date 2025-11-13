fn is_palindrome(x: i32) -> bool {
    let binding = x.to_string();
    let fwd = binding.chars();
    let rev = fwd.clone().rev();

    for (c1, c2) in fwd.zip(rev) {
        if c1 != c2 { return false };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_neg121() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn test_10() {
        assert_eq!(is_palindrome(10), false);
    }
}
