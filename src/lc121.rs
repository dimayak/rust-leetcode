// 121. Best Time to Buy and Sell Stock - https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best_profit = 0;
        let mut current_min = prices[0];

        for i in 1..prices.len() {
            let price = prices[i];

            if price < current_min {
                current_min = price;
            }

            if best_profit < price - current_min {
                best_profit = price - current_min
            }
        }


        best_profit
    }

    pub fn test() {
        let v1 = vec![7,1,5,3,6,4];
        let v2 = vec![7,6,4,3,1];
        let v3 = vec![1,2,3,4,5,6,7,8,9];

        assert_eq!(Self::max_profit(v1), 5);
        assert_eq!(Self::max_profit(v2), 0);
        assert_eq!(Self::max_profit(v3), 8);
    }
}