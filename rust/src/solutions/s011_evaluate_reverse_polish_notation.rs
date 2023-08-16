#![allow(dead_code, unused_variables)]

pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for token in tokens.into_iter() {
            match token.as_str() {
                "+" | "-" | "/" | "*" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();

                    if token == "+" {
                        stack.push(first + second);
                    }

                    if token == "*" {
                        stack.push(first * second);
                    }

                    if token == "-" {
                        stack.push(first - second);
                    }

                    if token == "/" {
                        stack.push(first / second);
                    }
                }
                _ => {
                    let num: i32 = token.parse().unwrap();
                    stack.push(num);
                }
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s011_evaluate_reverse_polish_notation::Solution;

    #[test]
    fn it_works() {
        let input_1 = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        let input_2 = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];
        let input_3 = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];

        assert_eq!(Solution::eval_rpn(input_1), 9);
        assert_eq!(Solution::eval_rpn(input_2), 6);
        assert_eq!(Solution::eval_rpn(input_3), 22);
    }
}
