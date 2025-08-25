
pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m:i32 = mat.len() as i32;
        let n:i32 = mat[0].len() as i32;
        let mut ret = vec![];
        let mut up = true;
        let mut r: i32 = 0;
        let mut c:i32 = 0;
        ret.push(mat[0][0]);
        while ret.len() < (m * n) as usize {
            if up {
                r -= 1;
                c += 1;
            } else {
                r += 1;
                c -= 1;
            }

            if r >= 0 && r < m && c >= 0 && c < n {
                ret.push(mat[r as usize][c as usize]);
                continue;
            }
            if r < 0 && c >= 0 && c < n {
                up = false;
                c += 1;
                continue;
            }
            if c < 0 && r >= 0 && r < m {
                up = true;
                r += 1;
                continue;
            }
            if r < 0 && c >= n {
                up = false;
                r += 1;
                continue;
            }
            if r >=m && c >= 0 && c < n {
                up = true;
                c+= 1;
                continue;
            }
            if r >= m && c < 0 {
                up = true;
                c += 1;
                continue;
            }
            if c >= n && r >=0 && r < m {
                up = false;
                r += 1;
                continue
            }
            break;
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
            vec![1,2,4,7,5,3,6,8,9],
            Solution::find_diagonal_order(vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ])
        );
        assert_eq!(
            vec![1,2,3,4],
            Solution::find_diagonal_order(vec![
                vec![1,2],
                vec![3,4],
            ])
        );
        assert_eq!(
            vec![3,2],
            Solution::find_diagonal_order(vec![
                vec![3],
                vec![2],
            ])
        );
    }
}
