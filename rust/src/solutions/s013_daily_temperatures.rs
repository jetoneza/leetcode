#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    // O(n^2)
    pub fn solution_1_daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        for i in 0..temperatures.len() {
            let temp = temperatures[i];
            let mut days = 0;

            for j in (i + 1)..=temperatures.len() {
                if j == temperatures.len() {
                    ans.push(0);
                    break;
                }

                let next_temp = temperatures[j];

                days += 1;

                if next_temp > temp {
                    ans.push(days);
                    break;
                }
            }
        }

        ans
    }

    // O(n)
    pub fn solution_2_daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = vec![];

        for (i, temp) in temperatures.into_iter().enumerate() {
            while let Some(last) = stack.last() {
                let (val, index) = last;
                let index = index.to_owned();

                if temp <= val.to_owned() {
                    break;
                }

                ans[index] = (i - index) as i32;
                stack.pop();
            }

            stack.push((temp, i));
        }

        ans
    }

    // O(n)
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = vec![];

        for i in (0..temperatures.len()).rev() {
            let temp = temperatures[i];

            while let Some(last) = stack.last() {
                if temp < last.0 {
                    break;
                }

                stack.pop();
            }

            if let Some(last) = stack.last() {
                if temp < last.0 {
                    ans[i] = (last.1 - i) as i32;
                }
            }

            stack.push((temp, i));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s013_daily_temperatures::Solution;

    #[test]
    fn it_works() {
        let input_1 = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let input_2 = vec![30, 40, 50, 60];
        let input_3 = vec![30, 60, 90];

        assert_eq!(
            Solution::daily_temperatures(input_1),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(Solution::daily_temperatures(input_2), vec![1, 1, 1, 0]);
        assert_eq!(Solution::daily_temperatures(input_3), vec![1, 1, 0]);
    }
}
