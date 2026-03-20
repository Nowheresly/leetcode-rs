use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let mut ans = vec![vec![0;n-k+1];m-k+1];

        for row in 0..m-k+1 {
            for col in 0..n-k+1 {
                let mut set = BTreeSet::new();
                for i in row..row+k {
                    for j in col..col+k {
                        set.insert(grid[i][j]);
                    }
                }
                let mut min = i32::MAX;
                let mut it = set.iter();
                let mut prev = it.next().unwrap();
                while let Some(cur) = it.next() {
                    min = min.min((cur - prev).abs());
                    prev = cur;
                }
                if min != i32::MAX {
                    ans[row][col] = min;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1,2]], Solution::min_abs_diff(vec![vec![1,-2,3],vec![2,3,5]],2));

    }
}
