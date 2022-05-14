// 303. Range Sum Query - Immutable - https://leetcode.com/problems/range-sum-query-immutable/
struct NumArray {
    sums: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len()];
        let mut sum = 0;
        for i in 0..sums.len() {
            sum += nums[i];
            sums[i] = sum;
        }
        Self { sums }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        let sum_right = self.sums[right];
        let mut sum_left = 0;
        if left > 0 {
            sum_left = self.sums[left - 1];
        }

        return sum_right - sum_left;
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

pub struct Solution;
impl Solution {
    pub fn test() {
        let na1 = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(na1.sum_range(0, 2), 1);
        assert_eq!(na1.sum_range(2, 5), -1);
        assert_eq!(na1.sum_range(0, 5), -3);
    }
}