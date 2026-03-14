use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut deq:VecDeque<String> = VecDeque::new();
        let mut index_in_sorted_list = 0;
        deq.push_back(String::from(""));

        while let Some(current_string) = deq.pop_back() {
            if current_string.len() == n {
                index_in_sorted_list += 1;
                if index_in_sorted_list == k {
                    return current_string;
                }
                continue;
            }
            for current_char in ('a'..='c').rev() {
                if current_string.len() == 0 || current_string.chars().last().unwrap() != current_char {
                    deq.push_back(current_string.clone() + &current_char.to_string());
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("c"), Solution::get_happy_string(1, 3));
    }
}
