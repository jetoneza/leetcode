#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut p_stack = vec![];
        let mut h_stack: Vec<i32> = vec![];
        let mut max = 0;

        for (i, &h) in heights.iter().enumerate() {
            let mut index = i;

            while let Some(last_h) = h_stack.last() {
                if h >= last_h.to_owned() {
                    break;
                }

                let last_p = p_stack[p_stack.len() - 1];

                let a = (i - last_p) as i32 * last_h;

                if max < a {
                    max = a
                }

                index = last_p;

                h_stack.pop();
                p_stack.pop();
            }

            h_stack.push(h);
            p_stack.push(index);
        }

        while let Some(last_h) = h_stack.last() {
            let last_p = p_stack[p_stack.len() - 1];

            let a = last_h * (heights.len() as i32 - last_p as i32);

            if max < a {
                max = a
            }

            h_stack.pop();
            p_stack.pop();
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s015_largest_rectangle_in_histogram::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
