pub struct Solution {}

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let uniqu = complexity[0];
        for i in 1..complexity.len() {
            if complexity[i] <= uniqu {
                return 0;
            }
        }
        let mut ans = 1i64;
        let modu = 1_000_000_007i64;
        for i in 2..complexity.len() as i64 {
            ans *= i;
            ans = ans % modu;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_permutations(vec![1,2,3]));
        assert_eq!(0, Solution::count_permutations(vec![3,3,3,4,4,4]));
    }
}
