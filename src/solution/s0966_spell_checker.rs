use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut exact = HashSet::new();
        let mut lower:HashMap<String, Vec<&str>> = HashMap::new();
        let mut wildcard:HashMap<String, Vec<&str>> = HashMap::new();

        for w in wordlist.iter() {
            exact.insert(w.clone());
            let key = w.clone().to_lowercase();
            if lower.contains_key(&key) {
                lower.get_mut(&key).unwrap().push(w.as_str());
            } else {
                lower.insert(key.clone(), vec![&w]);
            }
            let key = key.clone().replace("a", "*").replace("e", "*").replace("i", "*").replace("o", "*").replace("u", "*");
            if wildcard.contains_key(&key) {
                wildcard.get_mut(&key).unwrap().push(w.as_str());
            } else {
                wildcard.insert(key.clone(), vec![&w]);
            }
        }
        let mut res = vec![];
        for q in queries.iter() {
            if exact.contains(q) {
                res.push(q.clone());
                continue;
            }
            let low = q.to_lowercase();
            if lower.contains_key(&low) {
                res.push(lower.get(&low).unwrap()[0].to_string());
                continue;
            }
            let key = low.clone().replace("a", "*").replace("e", "*").replace("i", "*").replace("o", "*").replace("u", "*");
            if wildcard.contains_key(&key) {
                res.push(wildcard.get(&key).unwrap()[0].to_string());
                continue;
            } else {
                res.push(String::from(""));
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
        assert_eq!(vec![String::from("kite"),String::from("KiTe"),String::from("KiTe"),String::from("Hare"),String::from("hare"),String::from(""),String::from(""),String::from("KiTe"),String::from(""),String::from("KiTe")], Solution::spellchecker(vec![String::from("KiTe"),String::from("kite"),String::from("hare"),String::from("Hare")],
                                                                                                           vec![String::from("kite"),String::from("Kite"),String::from("KiTe"),String::from("Hare"),String::from("HARE"),String::from("Hear"),String::from("hear"),String::from("keti"),String::from("keet"),String::from("keto")]));

    }
}
