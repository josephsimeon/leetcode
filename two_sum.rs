fn two_sum (nums: Vec<i32>, target: i32) -> Vec<i32>
{
    for (i, n) in nums.iter().enumerate()
    {
        for (j, m) in nums.iter().enumerate()
        {
            let check_iterator = i != j;
            let check_sum = (n + m) == target;

            if check_iterator && check_sum
            {
                return vec![i as i32, j as i32];
            }
        }
    }

    return vec![0, 0];
}

#[test]
fn test1 ()
{
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test2 ()
{
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test3 ()
{
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
