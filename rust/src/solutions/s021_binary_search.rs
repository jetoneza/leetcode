#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let i = (l + r) / 2;
            let val = nums[i];

            if val == target {
                return i as i32;
            }

            if val < target && i < nums.len() {
                l = i + 1;
            } else if val > target && i > 0 {
                r = i - 1;
            } else {
                return -1;
            }
        }

        -1
    }

    // Recursion
    pub fn recursion_search(nums: Vec<i32>, target: i32) -> i32 {
        fn search_target(nums: &Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
            let i = (start + end) / 2;
            let val = nums[i as usize];

            if val == target {
                return i as i32;
            }

            if start == end {
                return -1;
            }

            if val < target && i < nums.len() as i32 {
                return search_target(&nums, target, i + 1, end);
            } else if val > target && i > 0 {
                return search_target(&nums, target, start, i - 1);
            } else {
                -1
            }
        }

        search_target(&nums, target, 0, (nums.len() - 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s021_binary_search::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![5], 5), 0);
        assert_eq!(Solution::search(vec![2, 5], 10), -1);
        assert_eq!(Solution::search(vec![5], 10), -1);
    }
}
