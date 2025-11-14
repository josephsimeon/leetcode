fn int_to_roman(num: i32) -> String {
    let mut s: String = String::new();

    let mut num: i32 = num;
    while num > 1 {
        let reduce;
        match num {
            num if num >= 1000 => {
                s.push_str("M");
                reduce = 1000;
            },
            num if num >= 500 => {
                s.push_str("D");
                reduce = 500;
            },
            num if num >= 100 => {
                s.push_str("C");
                reduce = 100;
            },
            num if num >= 50 => {
                s.push_str("L");
                reduce = 50;
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
    fn test_3769() {
        assert_eq!(int_to_roman(3769), "MMMDCCLXIX");
    }
}
