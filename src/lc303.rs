// 303. Range Sum Query - Immutable - https://leetcode.com/problems/range-sum-query-immutable/
struct NumArray {
    nums: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        Self { nums: nums }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        for i in left..=right {
            sum += self.nums[i as usize];
        }

        sum
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