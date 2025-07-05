pub struct Solution {}

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_ones = 0;
        let mut idx = 0;

        for (i, row) in mat.iter().enumerate() {
            let count = row.iter().filter(|&&x| x == 1).count() as i32;
            if count > max_ones {
                max_ones = count;
                idx = i as i32;
            }
        }

        return vec![idx, max_ones];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0,1], Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]));
        assert_eq!(vec![1,2], Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]));
        assert_eq!(vec![1,2], Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]));
    }
}