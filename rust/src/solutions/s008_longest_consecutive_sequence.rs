#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn old_longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut sorted = nums.clone();
        sorted.sort();

        let mut check = sorted[0]; // Current number to be checked
        let mut len = 1; // Current length of series
        let mut longest = 0; // Longest length of all the checked series

        for i in 0..sorted.len() {
            let num = sorted[i];

            if num == check {
                continue;
            }

            if num == check + 1 {
                len += 1;
                check = num;
                continue;
            }

            if longest < len {
                longest = len
            }

            check = num;
            len = 1;
        }

        if longest < len {
            longest = len
        }

        longest
    }

    // O(n) solution
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for i in 0..nums.len() {
            let num = nums[i];

            if set.get(&(num - 1)) == None {
                let mut len = 0;

                loop {
                    if set.get(&(num + len)) == None {
                        break;
                    }

                    len += 1;
                }

                if longest < len {
                    longest = len;
                }
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s008_longest_consecutive_sequence::Solution;

    #[test]
    fn it_works() {
        let input_1 = Vec::from([100, 4, 200, 1, 3, 2]);
        let input_2 = Vec::from([0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        let input_3 = Vec::from([]);

        assert_eq!(Solution::longest_consecutive(input_1), 4);
        assert_eq!(Solution::longest_consecutive(input_2), 9);
        assert_eq!(Solution::longest_consecutive(input_3), 0);
    }
}
