use std::collections::HashSet;

pub struct Solution {
}

pub struct Data {
    pub min:String,
    pub visited:HashSet<String>,
}

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut data = Data {
            min: s.clone(),
            visited: HashSet::new(),
        };
        Self::dfs(&mut data, s, a as usize, b as usize);
        data.min
    }

    pub fn dfs(data:&mut Data, s: String, a: usize, b: usize) -> () {
        if data.visited.contains(&s) {
            return;
        }
        data.visited.insert(s.clone());
        let n = s.len();
        // rotate
        let mut to = String::new();
        to.push_str(&s[b..n]);
        to.push_str(&s[0..b]);
        if to < data.min {
            data.min = to.clone();
        }
        Self::dfs(data, to, a, b);
        // add
        let mut chars:Vec<char> = s.chars().collect();
        for i in (1..n).step_by(2) {
            let digit = chars[i].to_digit(10).unwrap();
            let new_digit = (digit + a as u32) % 10;
            chars[i] = std::char::from_digit(new_digit, 10).unwrap();
        }
        let to:String = chars.iter().collect();
        if to < data.min {
            data.min = to.clone();
        }
        Self::dfs(data, to, a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("2050"), Solution::find_lex_smallest_string(String::from("5525"), 9, 2));
        assert_eq!(String::from("24"), Solution::find_lex_smallest_string(String::from("74"), 5, 1));
        assert_eq!(String::from("0011"), Solution::find_lex_smallest_string(String::from("0011"), 4, 2));
    }
}
