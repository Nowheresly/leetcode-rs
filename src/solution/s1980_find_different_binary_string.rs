pub struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut res = String::new();
        for i in 0..nums[0].len() {
            if nums[i].chars().nth(i).unwrap() == '1' {
                res.push('0');
            } else {
                res.push('1');
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("101"),
            Solution::find_different_binary_string(vec![
                String::from("111"),
                String::from("011"),
                String::from("001")
            ])
        );
    }
}
