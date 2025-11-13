use std::cmp::min;

fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0;

    for (i, x) in height.iter().enumerate() {
        for (j, y) in height.iter().enumerate() {
            if i != j {
                let size = min(x, y) * (i as i32 - j as i32).abs();
                if size > area { area = size };
            }
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
