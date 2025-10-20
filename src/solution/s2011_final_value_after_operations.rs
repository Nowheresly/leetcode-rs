
pub struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;
        for operation in operations {
            if operation.contains('+') {
                res += 1;
            } else {
                res -= 1;
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

        assert_eq!(1, Solution::final_value_after_operations(vec![String::from("--X"), String::from("++X"), String::from("++X")]));
        assert_eq!(3, Solution::final_value_after_operations(vec![String::from("++X"), String::from("++X"), String::from("X++")]));
        assert_eq!(0, Solution::final_value_after_operations(vec![String::from("X++"), String::from("++X"), String::from("--X"), String::from("X--")]));
    }
}
