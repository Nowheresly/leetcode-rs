
pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if stack.is_empty() == false && stack[stack.len()-1] == c {
                stack.pop().unwrap();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("ay"), Solution::remove_duplicates(String::from("azxxzy")));
    }
}
