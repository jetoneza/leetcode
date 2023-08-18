#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    // Brute force solution using two pointers, this is different from the
    // HashMap technique used in two_sum.
    pub fn solution_1_two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = 1;

        loop {
            let sum = numbers[left] + numbers[right];

            if sum == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }

            if sum > target || right == numbers.len() - 1 {
                left += 1;

                if right == left {
                    break;
                }

                right = left + 1;

                continue;
            }

            right += 1;
        }

        vec![]
    }

    // A more efficient solution where the right pointer starts at the end
    // of the array.
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s017_two_sum_sorted::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![5, 25, 75], 100), vec![2, 3]);
    }
}
