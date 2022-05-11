// 53. Maximum Subarray - https://leetcode.com/problems/maximum-subarray/
pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut best_sum = i32::MIN;

        for i in 0..nums.len() {
            sum = nums[i].max(nums[i] + sum);
            if sum > best_sum {
                best_sum = sum;
            }
        }

        best_sum
    }
    pub fn test() {
        let v1 = vec![-2,1,-3,4,-1,2,1,-5,4];
        let v2 = vec![1];
        let v3 = vec![5,4,-1,7,8];

        assert_eq!(Self::max_sub_array(v1), 6);
        assert_eq!(Self::max_sub_array(v2), 1);
        assert_eq!(Self::max_sub_array(v3), 23);
    }
}