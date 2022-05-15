// 338. Counting Bits - https://leetcode.com/problems/counting-bits/
pub struct Solution;

/*
    0 - 0000 - 0
    1 - 0001 - 1
    2 - 0010 - 1
    3 - 0011 - 2
    4 - 0100 - 1
    5 - 0101 - 2
    6 - 0110 - 2
    7 - 0111 - 3
    8 - 1000 - 1
    9 - 1001 - 2
    10 - 1010 - 2
    11 - 1011 - 3
    12 - 1100 - 2
    13 - 1101 - 3
    14 - 1110 - 3
    15 - 1111 - 4
*/

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; (n + 1) as usize];

        for i in 0..=n {
            result[i as usize] = i.count_ones() as i32;
        }

        result
    }

    pub fn test() {
        assert_eq!(Self::count_bits(2), vec![0,1,1]);
        assert_eq!(Self::count_bits(5), vec![0,1,1,2,1,2]);
    }
}