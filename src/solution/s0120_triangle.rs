
pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut triangle = triangle;
        for h in (1..n).rev() {
            let base = &triangle[h];
            let mut sup = triangle[h-1].clone();
            for i in 0..base.len() - 1 {
                let topi = i;
                let base1 = i;
                let base2 = i + 1;
                let val = base[base1].min(base[base2]);
                sup[topi] = sup[topi] + val;
            }
            triangle[h-1] = sup;
        }

        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(11, Solution::minimum_total(vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));

    }
}
