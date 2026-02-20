
pub struct Solution {}

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        if s.eq("10") {
            return s;
        }
        let mut count = 0;
        let mut start = 0;
        let ch = s.chars().collect::<Vec<char>>();
        let mut list = vec![];
        for i in 0..ch.len() {
            if ch[i] == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                let str = s[start..i+1].to_string();
                list.push(str);
                start = i+1;
            }
        }
        if list.len() == 1 {
            let str = list[0].to_string()[1..list[0].len()-1].to_string();
            return format!("1{}0", Self::make_largest_special(str));
        }
        list = list.iter().map(|x| Self::make_largest_special(x.to_string())).collect();
        list.sort_by(|a, b| b.cmp(&a));
        let mut res = String::new();
        for str in list {
            res.push_str(&str);
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
            "11100100",
            Solution::make_largest_special(String::from("11011000"))
        );
    }
}
