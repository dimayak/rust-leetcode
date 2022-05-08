// 268. Missing number - https://leetcode.com/problems/missing-number/
pub struct Solution;
impl Solution {
    // Runtime: 3 ms, faster than 67.86% of Rust online submissions for Missing Number.
    // Memory Usage: 2.4 MB, less than 13.39% of Rust online submissions for Missing Number.
    pub fn _missing_number(nums: Vec<i32>) -> i32 {
        let nums_sum: i32 = nums.iter().sum();
        let n_sum: i32 = (0..=nums.len() as i32).sum();

        n_sum - nums_sum
    }

    // Runtime: 0 ms, faster than 100.00% of Rust online submissions for Missing Number.
    // Memory Usage: 2.4 MB, less than 12.50% of Rust online submissions for Missing Number.
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut sum = len * (len + 1) / 2;

        for i in nums.iter() {
            sum -= i;
        }

        sum
    }

    pub fn test() {
        let t1= vec![3,0,1];
        let t2 = vec![0,1];
        let t3 = vec![9,6,4,2,3,5,7,0,1];
    
        assert_eq!(Self::missing_number(t1), 2);
        assert_eq!(Self::missing_number(t2), 2);
        assert_eq!(Self::missing_number(t3), 8);
    }
}