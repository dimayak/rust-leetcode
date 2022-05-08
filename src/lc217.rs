use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn _contains_duplicate_insert(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for num in nums.iter() {
            if !set.insert(num) {
                return true;
            }
        }

        false
    }

    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return true;
            }
        }

        false
    }

    pub fn test() {
        let t1= vec![1,2,3,1];
        let t2 = vec![1,2,3,4];
        let t3 = vec![1,1,1,3,3,4,3,2,4,2];
    
        assert_eq!(Self::contains_duplicate(t1), true);
        assert_eq!(Self::contains_duplicate(t2), false);
        assert_eq!(Self::contains_duplicate(t3), true);
    }
}