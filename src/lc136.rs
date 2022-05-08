// 136. Single Number - https://leetcode.com/problems/single-number/
pub struct Solution;

impl Solution {
    // Naive solution
    // Runtime: 3 ms, faster than 64.00% of Rust online submissions for Single Number.
    // Memory Usage: 2.3 MB, less than 60.57% of Rust online submissions for Single Number.
    pub fn _single_number(nums: Vec<i32>) -> i32 {
        let mut n = nums[0];
        let len = nums.len();
        for i in 1..len {
            n = n ^ nums[i]
        }

        n
    }

    // oneliner
    // Runtime: 3 ms, faster than 64.00% of Rust online submissions for Single Number.
    // Memory Usage: 2.2 MB, less than 90.29% of Rust online submissions for Single Number.
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, i| acc ^ i)
    }

    pub fn test() {
        let t1 = vec![2, 2, 1];
        let t2 = vec![4,1,2,1,2];
        let t3 = vec![1];
        assert_eq!(Self::single_number(t1), 1);
        assert_eq!(Self::single_number(t2), 4);
        assert_eq!(Self::single_number(t3), 1);
    }
}