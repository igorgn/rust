fn main() {
    assert_eq!(count_even_subarrays(vec![2, 4, 1, 6, 8, 10], 2), 3);
assert_eq!(count_even_subarrays(vec![1, 3, 5, 7], 2), 0);
assert_eq!(count_even_subarrays(vec![2, 4, 6, 8], 3), 2);
assert_eq!(count_even_subarrays(vec![2], 1), 1);
assert_eq!(count_even_subarrays(vec![], 2), 0);
}

fn count_even_subarrays(nums: Vec<i32>, k: usize) -> i32 {
    let mut left = 0;
    
    todo!()
}