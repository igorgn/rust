use std::collections::{HashMap, HashSet};

fn main() {}

fn has_pair_with_sum(nums: &[i32], target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len().saturating_sub(1);

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return true;
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    false
}

fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.len() < 2 {
        return nums.len();
    }

    let mut left = 1;
    for right in 1..nums.len() {
        if nums[left - 1] != nums[right] {
            nums[left] = nums[right];
            left += 1;
        }
    }
    left
}

fn move_negatives_to_front(nums: &mut [i32]) {
    let mut left = 0;

    for i in 0..nums.len() {
        if nums[i] < 0 {
            nums.swap(left, i);
            left += 1;
        }
    }
}

fn longest_unique_subarray(nums: &[i32]) -> usize {
    if nums.len() < 2 {
        return nums.len();
    }

    let mut length = 0;
    let mut left = 0;
    for right in 1..nums.len() {
        if nums[right - 1] == nums[right] {
            length = length.max(right - left);
            left = right;
        }
    }
    length.max(nums.len() - left)
}

fn count_unique_pairs_with_sum(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len().saturating_sub(1);
    let mut hs = HashSet::new();

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            hs.insert((nums[left], nums[right]));
            right -= 1;
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    hs.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    // count_unique_pairs_with_sum
    #[test]
    fn test_basic_pairs() {
        let nums = [1, 2, 2, 3, 4, 4, 5];
        assert_eq!(count_unique_pairs_with_sum(&nums, 6), 2);
    }

    #[test]
    fn test_no_pairs() {
        let nums = [1, 2, 3];
        assert_eq!(count_unique_pairs_with_sum(&nums, 10), 0);
    }

    #[test]
    fn test_all_same() {
        let nums = [2, 2, 2, 2];
        assert_eq!(count_unique_pairs_with_sum(&nums, 4), 1);
    }

    #[test]
    fn test_empty() {
        let nums: [i32; 0] = [];
        assert_eq!(count_unique_pairs_with_sum(&nums, 5), 0);
    }

    #[test]
    fn test_single_element() {
        let nums = [7];
        assert_eq!(count_unique_pairs_with_sum(&nums, 14), 0);
    }
    //has_pair_with_sum
    #[test]
    fn test_pair_exists() {
        assert!(has_pair_with_sum(&[1, 2, 4, 7, 11, 15], 15));
        assert!(has_pair_with_sum(&[1, 2, 3, 4, 5], 9));
    }

    #[test]
    fn test_pair_not_exists() {
        assert!(!has_pair_with_sum(&[1, 2, 3, 4, 5], 10));
        assert!(!has_pair_with_sum(&[1, 2, 3], 7));
    }

    #[test]
    fn test_empty_and_single() {
        assert!(!has_pair_with_sum(&[], 5));
        assert!(!has_pair_with_sum(&[1], 2));
    }

    //remove_duplicates
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 3];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 3);
        assert_eq!(&nums[..len], &[1, 2, 3]);
    }

    #[test]
    fn test_no_duplicates() {
        let mut nums = vec![1, 2, 3];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 3);
        assert_eq!(&nums[..len], &[1, 2, 3]);
    }

    #[test]
    fn test_all_duplicates() {
        let mut nums = vec![2, 2, 2, 2];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 1);
        assert_eq!(&nums[..len], &[2]);
    }

    #[test]
    fn test_empty3() {
        let mut nums = vec![];
        let len = remove_duplicates(&mut nums);
        assert_eq!(len, 0);
    }

    //move_negatives_to_front
    fn is_partitioned(nums: &[i32]) -> bool {
        let mut found_positive = false;
        for &num in nums {
            if num < 0 && found_positive {
                return false;
            }
            if num >= 0 {
                found_positive = true;
            }
        }
        true
    }

    #[test]
    fn test_mixed() {
        let mut nums = [4, -1, 9, -3, 5, -2];
        move_negatives_to_front(&mut nums);
        assert!(is_partitioned(&nums));
    }

    #[test]
    fn test_all_negative() {
        let mut nums = [-5, -2, -3];
        move_negatives_to_front(&mut nums);
        assert!(is_partitioned(&nums));
    }

    #[test]
    fn test_all_positive() {
        let mut nums = [1, 2, 3];
        move_negatives_to_front(&mut nums);
        assert!(is_partitioned(&nums));
    }

    #[test]
    fn test_empty2() {
        let mut nums: [i32; 0] = [];
        move_negatives_to_front(&mut nums);
        assert!(is_partitioned(&nums));
    }

    //
    #[test]
    fn test_longest_unique_subarray_basic() {
        let nums = [1, 2, 2, 3, 4, 4, 5];
        assert_eq!(longest_unique_subarray(&nums), 3);
    }

    #[test]
    fn test_all_unique() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(longest_unique_subarray(&nums), 5);
    }

    #[test]
    fn test_all_duplicates2() {
        let nums = [2, 2, 2, 2];
        assert_eq!(longest_unique_subarray(&nums), 1);
    }

    #[test]
    fn test_empty_array() {
        let nums: [i32; 0] = [];
        assert_eq!(longest_unique_subarray(&nums), 0);
    }

    #[test]
    fn test_single_element1() {
        let nums = [7];
        assert_eq!(longest_unique_subarray(&nums), 1);
    }
}
