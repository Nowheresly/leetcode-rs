
pub struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut area_max = 0;
        let mut diag_max = 0;
        for i in 0..dimensions.len() {
            let l = dimensions[i][0];
            let h = dimensions[i][1];
            let diag = l * l + h * h;
            if diag > diag_max || diag == diag_max && l * h > area_max {
                diag_max = diag;
                area_max = l * h;
            }
        }
        area_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(48, Solution::area_of_max_diagonal(vec![vec![9,3],vec![8,6]]));
        assert_eq!(12, Solution::area_of_max_diagonal(vec![vec![3,4],vec![4,3]]));
        assert_eq!(30, Solution::area_of_max_diagonal(vec![vec![2,6],vec![5,1],vec![3,10],vec![8,4]]));
    }
}
