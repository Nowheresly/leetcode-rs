pub struct Solution {}

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let n = n - 1;
        let rounds = k / n;
        let rem = k % n;

        if rounds % 2 == 0 {
            return rem;
        }
        n - rem
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::number_of_child(3, 5));
        assert_eq!(2, Solution::number_of_child(5, 6));

    }
}