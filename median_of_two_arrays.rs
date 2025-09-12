fn find_median_sorted_arrays (nums1: Vec<i32>, nums2: Vec<i32>) -> f64
{
    let mut nums: Vec<i32> = Vec::new();

    if nums1.len() > 0
    {
        nums.extend(nums1.iter().copied());
    }

    if nums2.len() > 0
    {
        nums.extend(nums2.iter().copied());
    }

    if nums.len() <= 1
    {
        return nums[0] as f64;
    }

    nums.sort();

    let median: f64;
    let limit = nums.len();
    match limit % 2
    {
        0 => {
            median = (nums[(limit / 2) - 1] + nums[limit / 2]) as f64 / 2.0;
        },
        _ => {
            median = nums[(limit - 1) / 2] as f64 / 1.0;
        },
    }

    median
}

#[test]
fn test1 ()
{
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
}

#[test]
fn test2 ()
{
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}

#[test]
fn test3 ()
{
    assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]), 9.0);
}

#[test]
fn test4 ()
{
    assert_eq!(find_median_sorted_arrays(vec![2, 2, 4, 4], vec![2, 2, 2, 4, 4]), 2.0);
}
