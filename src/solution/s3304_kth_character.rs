pub struct Solution {}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut str:Vec<char> = vec!['a'];
        
        while str.len() < k as usize {
            let mut new_str:Vec<char> = vec![];
            for c in &str {
                if *c == 'z' {
                    new_str.push('a');
                } else {
                    new_str.push(((*c as u8) + 1) as char);
                }
            }
            str.append(&mut new_str);
        }
        
        return str.iter().nth(k as usize - 1).unwrap_or(&'a').clone();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!('b', Solution::kth_character(5));
        assert_eq!('c', Solution::kth_character(10));
    }
}