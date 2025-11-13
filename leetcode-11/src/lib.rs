use std::cmp::{max, min};

fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0;
    let mut i = 0;
    let mut j = height.len() - 1;

    while i < j {
        let x = height[i];
        let y = height[j];

        area = max(area, min(x, y) * (j as i32 - i as i32).abs());

        if x < y {
            i += 1;
        } else {
            j -= 1;
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(test), 49);
    }

    #[test]
    fn test_2() {
        let test: Vec<i32> = vec![1, 1];
        assert_eq!(max_area(test), 1);
    }
}
