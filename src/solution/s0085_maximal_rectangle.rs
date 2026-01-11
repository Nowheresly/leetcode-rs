pub struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut res = 0;
        let mut heights = vec![0; cols + 1];

        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[col] += 1;
                } else {
                    heights[col] = 0;
                }
            }
            let mut stack = vec![-1];
            for i in 0..=cols {
                while *stack.last().unwrap() != -1 && heights[*stack.last().unwrap() as usize] >= heights[i] {
                    let h = heights[stack.pop().unwrap() as usize];
                    let w = i as i32 - *stack.last().unwrap() - 1;
                    res = res.max(h * w);
                }
                stack.push(i as i32);
            }
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::maximal_rectangle(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']]));
        

    }
}
