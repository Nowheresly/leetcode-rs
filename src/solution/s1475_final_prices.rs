pub struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; prices.len()];
        let mut stack:Vec<usize> = vec![];

        for i in 0..prices.len() {
            let val = prices[i];
            answer[i] = val;
            while stack.len() > 0 && val <= prices[stack[stack.len() - 1]] {
                let idx = stack.pop().unwrap();
                answer[idx] = prices[idx] - val;
            }
            stack.push(i);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![4, 2, 4, 2, 3],
            Solution::final_prices(vec![8, 4, 6, 2, 3])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            Solution::final_prices(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(vec![9, 0, 1, 6], Solution::final_prices(vec![10, 1, 1, 6]));
    }
}
