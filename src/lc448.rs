use std::mem;

pub struct Solution;
impl Solution {
    // Runtime: 18 ms, faster than 46.91% of Rust online submissions for Find All Numbers Disappeared in an Array.
    // Memory Usage: 2.6 MB, less than 81.48% of Rust online submissions for Find All Numbers Disappeared in an Array.
    // Time complexity: O(n)
    // Space complexity: no extra space (return list doesn't count)
    pub fn _find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // Make and array of zeroes larger than nums by 1
        let mut result = vec![0i32; nums.len() + 1];

        // After this code result[i] == 0 if the number is missing
        for i in nums.iter() {
            result[*i as usize] = *i;
        }

        // revert 0 to indexes and vise versa
        for i in 0..result.len() {
            if result[i] == 0 {
                result[i] = i as i32;
            }
            else {
                result[i] = 0;
            }
        }

        // remove zeroes
        result.retain(|&x| x > 0);

        result
    }

    // Runtime: 11 ms, faster than 85.19% of Rust online submissions for Find All Numbers Disappeared in an Array.
    // Memory Usage: 2.7 MB, less than 81.48% of Rust online submissions for Find All Numbers Disappeared in an Array.
    // Time complexity: O(n)
    // Space complexity: no extra space (return list doesn't count)
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0i32; n + 1];

        // After this code result[i] == 0 if the number is missing
        for i in 0..n {
            let index = nums[i];
            result[index as usize] = index;
        }

        // revert 0 to indexes and vise versa
        for i in 0..n + 1 {
            if result[i] == 0 {
                result[i] = i as i32;
            }
            else {
                result[i] = 0;
            }
        }

        // Move non zero elements to the beginning
        let mut pos = 0;
        for i in 1..result.len() {
            if result[i] > 0 {
                result[pos] = result[i];
                result[i] = 0;
                pos += 1;
            }
        }

        // Remove zeroes
        result.truncate(pos);

        result
    }

    pub fn test() {
        let t1 = vec![4,3,2,7,8,2,3,1];
        let t2 = vec![1,1];
        assert_eq!(Self::find_disappeared_numbers(t1), [5, 6]);
        assert_eq!(Self::find_disappeared_numbers(t2), [2]);
    }
}