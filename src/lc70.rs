// 70. Climbing Stairs - https://leetcode.com/problems/climbing-stairs/
pub struct Solution;

impl Solution {
    // Runtime: 0 ms, faster than 100.00% of Rust online submissions for Climbing Stairs.
    // Memory Usage: 1.9 MB, less than 94.01% of Rust online submissions for Climbing Stairs.
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut lookup = vec![1,2];

        for i in 2..n {
            lookup.push(lookup[i - 1] + lookup[i - 2]);
        }

        lookup[n-1]
    }

    pub fn test() {
        let n1 = 2;
        let n2 = 3;
        assert_eq!(Self::climb_stairs(n1), 2);
        assert_eq!(Self::climb_stairs(n2), 3);
    }
}