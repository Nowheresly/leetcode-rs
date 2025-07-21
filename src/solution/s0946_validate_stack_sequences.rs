
pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = std::collections::VecDeque::new();
        let mut i = 0;
        stack.push_back(pushed[i]);
        i += 1;
        let mut j = 0;

        while i < pushed.len() {
            if !stack.is_empty() && stack.back().unwrap() == &popped[j] {
                while stack.is_empty() == false && stack.back().unwrap() == &popped[j] {
                    stack.pop_back();
                    j += 1;
                }
            }
            stack.push_back(pushed[i]);
            i += 1;
        }

        while j < popped.len() {
            if stack.is_empty() || stack.back().unwrap() != &popped[j] {
                return false;
            }
            stack.pop_back();
            j += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]));
        assert_eq!(false, Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,3,5,1,2]));
    }
}
