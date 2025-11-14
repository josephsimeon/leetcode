fn int_to_roman(num: i32) -> String {
    let mut s: String = String::new();

    let mut num: i32 = num;
    while num > 0 {
        let reduce;
        match num {
            num if num >= 1000 => {
                s.push_str("M");
                reduce = 1000;
            },
            num if num >= 900 => {
                s.push_str("CM");
                reduce = 900;
            },
            num if num >= 500 => {
                s.push_str("D");
                reduce = 500;
            },
            num if num >= 400 => {
                s.push_str("CD");
                reduce = 400;
            },
            num if num >= 100 => {
                s.push_str("C");
                reduce = 100;
            },
            num if num >= 90 => {
                s.push_str("XC");
                reduce = 90;
            },
            num if num >= 50 => {
                s.push_str("L");
                reduce = 50;
            },
            num if num >= 40 => {
                s.push_str("XL");
                reduce = 40;
            },
            num if num >= 10 => {
                s.push_str("X");
                reduce = 10;
            },
            num if num >= 9 => {
                s.push_str("IX");
                reduce = 9;
            },
            num if num >= 5 => {
                s.push_str("V");
                reduce = 5;
            },
            num if num >= 4 => {
                s.push_str("IV");
                reduce = 4;
            },
            num if num >= 1 => {
                s.push_str("I");
                reduce = 1;
            },
            _ => reduce = 0,
        }

        num -= reduce;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3749() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn test_58() {
        assert_eq!(int_to_roman(58), "LVIII");
    }

    #[test]
    fn test_1994() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
