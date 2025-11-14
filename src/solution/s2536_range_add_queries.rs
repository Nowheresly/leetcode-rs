pub struct Solution {}

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut delta = vec![vec![0;n + 1]; n + 1];

        for q in queries {
            let row_start = q[0] as usize;
            let row_end = q[2] as usize;
            let col_start = q[1] as usize;
            let col_end = q[3] as usize;
            for i in row_start..=row_end {
                delta[i][col_start] += 1;
                delta[i][col_end + 1] -= 1;
            }
        }

        let mut ret = vec![vec![0; n]; n];
        let mut cur = 0;
        for i in 0..n {
            for j in 0..=n {
                cur += delta[i][j];
                if j == n {
                    continue;
                }
                ret[i][j] = cur;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]],
            Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]])
        );
    }
}
