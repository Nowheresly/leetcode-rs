
pub struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut k = k as usize;

        for c in num.chars() {
            while k > 0 && !stack.is_empty() && stack.last().unwrap() > &c {
                stack.pop();
                k -= 1;
            }
            stack.push(c);
        }

        // If we still have digits to remove, remove from the end
        while k > 0 && !stack.is_empty() {
            stack.pop();
            k -= 1;
        }

        // Convert stack to string and remove leading zeros
        let result: String = stack.into_iter().collect();
        let trimmed_result = result.trim_start_matches('0');

        if trimmed_result.is_empty() {
            "0".to_string()
        } else {
            trimmed_result.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "1219".to_string(),
            Solution::remove_kdigits(String::from("1432219"), 3)
        ); assert_eq!(
            "200".to_string(),
            Solution::remove_kdigits(String::from("10200"), 1)
        );
    }


}
