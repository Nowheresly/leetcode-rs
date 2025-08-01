
pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        ret.push(vec![1]);
        if num_rows == 1 {
            return ret;
        }
        ret.push(vec![1,1]);
        if num_rows == 2 {
            return ret;
        }
        for i in 2..num_rows as usize {
            let mut row = vec![1; i + 1];
            for j in 1..i {
                row[j] = ret[i - 1][j - 1] + ret[i - 1][j];
            }
            ret.push(row);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
        assert_eq!(vec![vec![1]], Solution::generate(1));
    }
}
