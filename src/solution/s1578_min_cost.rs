pub struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut sum = needed_time[0];
        let mut max = needed_time[0];
        let chars: Vec<char> = colors.chars().collect();
        for i in 1..chars.len() {
            if chars[i] != chars[i - 1] {
                ret += sum - max;
                sum = needed_time[i];
                max = needed_time[i];
            } else {
                sum += needed_time[i];
                max = max.max(needed_time[i]);
            }
        }
        ret += sum - max;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_cost(String::from("abaac"), vec![1,2,3,4,5]));
    }
}
