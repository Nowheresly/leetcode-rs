use std::collections::HashSet;

pub struct Solution {
}

impl Solution {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let n = text.len();
        let mut wset = HashSet::new();
        for w in words {
            wset.insert(w);
        }
        let mut res = vec![];
        for i in 0..n {
            for j in i..n {
                if wset.contains(&text[i..=j].to_string()) {
                    res.push(vec![i as i32, j as i32]);
                }
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
        assert_eq!(vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4]], Solution::index_pairs(String::from("ababa"), vec![String::from("aba"), String::from("ab")]));

    }
}
