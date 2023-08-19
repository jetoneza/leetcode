#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut nums = nums.to_owned();
        nums.sort_unstable();

        let len = nums.len();

        for i in 0..len {
            let v = nums[i];

            if i > 0 && v == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = len - 1;

            while l < r {
                let sum = nums[l] + nums[r] + v;

                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    ans.push(vec![v, nums[l].to_owned(), nums[r].to_owned()]);
                    l += 1;

                    while nums[l] == nums[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s018_3sum::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![-3, 3, 4, -3, 1, 2]),
            vec![vec![-3, 1, 2]]
        );
    }
}
