use std::collections::HashMap;

pub struct Solution {}

pub struct Data {
    ret: Vec<Vec<String>>,
    map: HashMap<String, Vec<String>>,
    n: usize,
}

impl Data {
    pub fn dfs(&mut self, words: &Vec<String>, cur: &mut Vec<String>, start: i32) {
        if cur.len() == self.n {
            self.ret.push(cur.to_vec());
            return;
        }
        if start >= words.len() as i32 {
            return;
        }
        if cur.is_empty() {
            self.dfs(words, cur, start + 1);
            cur.push(words[start as usize].to_string());
            self.dfs(words, cur, start);
            cur.remove(cur.len() - 1);
            return;
        }
        let next = self.find_next(cur);
        for ne in next {
            cur.push(ne.to_string());
            self.dfs(words, cur, start);
            cur.remove(cur.len() - 1);
        }
    }

    fn find_next(&mut self, cur: &Vec<String>) -> Vec<String> {
        let mut start = "";
        let mut tmp = String::new();
        if cur.len() == 1 {
            tmp.push(cur.get(0).unwrap().chars().nth(1).unwrap());
            start = &tmp;
        } else if cur.len() == 2 {
            tmp.push(cur.get(0).unwrap().chars().nth(2).unwrap());
            tmp.push(cur.get(1).unwrap().chars().nth(2).unwrap());
            start = &tmp;
        } else if cur.len() == 3 {
            tmp.push(cur.get(0).unwrap().chars().nth(3).unwrap());
            tmp.push(cur.get(1).unwrap().chars().nth(3).unwrap());
            tmp.push(cur.get(2).unwrap().chars().nth(3).unwrap());
            start = &tmp;
        }

        self.map.entry(start.to_string()).or_default().to_vec()
    }
}

impl Solution {
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let n = words.get(0).map(|x| x.len()).unwrap_or(0);
        let mut sol = Data {
            ret: vec![],
            map: HashMap::new(),
            n,
        };

        for w in &words {
            for i in 0..n {
                let prefix = w[0..i].to_string();
                sol.map.entry(prefix).or_insert(vec![]).push(w.to_string());
            }
        }
        let mut cur = vec![];
        sol.dfs(&words, &mut cur, 0);
        sol.ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                vec![
                    String::from("ball"),
                    String::from("area"),
                    String::from("lead"),
                    String::from("lady"),
                ],
                vec![
                    String::from("wall"),
                    String::from("area"),
                    String::from("lead"),
                    String::from("lady"),
                ],
            ],
            Solution::word_squares(vec![
                String::from("area"),
                String::from("lead"),
                String::from("wall"),
                String::from("lady"),
                String::from("ball")
            ])
        );
    }

    #[test]
    fn test_2() {
        let mut ret = Solution::word_squares(vec![
            String::from("abat"),
            String::from("baba"),
            String::from("atan"),
            String::from("atal"),
        ]);
        ret.sort();
        assert_eq!(
            vec![
                vec![
                    String::from("baba"),
                    String::from("abat"),
                    String::from("baba"),
                    String::from("atal"),
                ],
                vec![
                    String::from("baba"),
                    String::from("abat"),
                    String::from("baba"),
                    String::from("atan"),
                ],
            ],
            ret
        );
    }
}
