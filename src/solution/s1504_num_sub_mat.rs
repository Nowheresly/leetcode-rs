pub struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut res = 0;
        let mut height = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    height[j] = 0;
                } else {
                    height[j] = height[j] + 1;
                }
            }
            let mut stack: Vec<i32> = vec![];
            stack.push(-1);
            let mut sub_res: Vec<i32> = vec![0; n];
            for j in 0..n {
                while stack.len() > 1 && height[j] <= height[*stack.last().unwrap() as usize] {
                    stack.pop();
                }
                let k = *stack.last().unwrap();
                sub_res[j] = (j as i32 - k as i32) * height[j];
                if k != -1 {
                    sub_res[j] += sub_res[k as usize];
                }
                stack.push(j as i32);
            }
            res += sub_res.iter().sum::<i32>();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            13,
            Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
        );
    }
}
