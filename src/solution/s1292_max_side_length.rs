pub struct Solution {}

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1]
                    + (mat[i - 1][j - 1] as i32);
            }
        }

        let side_max = m.min(n);
        let mut res = 0;
        for i in 1..=m {
            for j in 1..=n {
                for c in (res + 1)..=side_max {
                    if i + c - 1 <= m && j + c - 1 <= n && get_rect(&prefix, i, j, i + c - 1, j + c - 1) <= threshold {
                        res += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        res as i32
    }
}

fn get_rect(prefix: &Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    prefix[x2][y2] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1] + prefix[x1 - 1][y1 - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::max_side_length(
                vec![
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2]
                ],
                4
            )
        );
    }
}
