#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    // My answer
    // This solution timeouts when input is very large.
    pub fn old_product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();

        for i in 0..nums.len() {
            let mut chunk = nums.to_owned();
            chunk.remove(i);

            let result = chunk.iter().fold(1, |acc, &val| acc * val);

            answer.push(result)
        }

        answer
    }

    // Correct answer based on:
    // https://www.youtube.com/watch?v=bNvIQI2wAjk
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut left = 1;

        for i in 0..nums.len() {
            answer.push(left);
            left *= nums[i];
        }

        let mut right = 1;

        for i in (0..nums.len()).rev() {
            answer[i] *= right;
            right *= nums[i];
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s006_product_of_array_except_self::Solution;

    #[test]
    fn it_works() {
        let input_1 = Vec::from([1, 2, 3, 4]);
        let input_2 = Vec::from([-1, 1, 0, -3, 3]);

        assert_eq!(Solution::product_except_self(input_1), vec![24, 12, 8, 6]);
        assert_eq!(Solution::product_except_self(input_2), vec![0, 0, 9, 0, 0]);
    }
}
