#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min = prices[0];

        for i in 0..prices.len() {
            let p = prices[i];

            if min > p {
                min = p;
            }

            let pr = p - min;

            if profit < pr {
                profit = pr;
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s022_max_profit::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 4, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
    }
}
