
pub struct Solution {}

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        for word in words.iter() {
            res.push(word.clone());
        }
        let mut hwm: usize = 0;
        loop {
            let mut updated = false;
            for i in hwm..(res.len() - 1) {
                let w1 = &res[i];
                let w2 = &res[i + 1];
                if is_anagram(w1, w2) {
                    res.remove(i + 1);
                    hwm = i;
                    updated = true;
                    break;
                }
            }
            if updated == false {
                break;
            }
        }
        res
    }
}

fn is_anagram(w1: &String, w2: &String) -> bool {
    if w1.len() != w2.len() {
        return false;
    }
    let mut count = vec![0; 26];
    for b in w1.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }
    for b in w2.as_bytes() {
        count[(b - b'a') as usize] -= 1;
    }
    for c in count.iter() {
        if *c != 0 {
            return false;
        }
    }
    true
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(vec![String::from("abba"), String::from("cd")],
                   Solution::remove_anagrams(vec![String::from("abba"), String::from("baba"),
                                                  String::from("cd"), String::from("cd")]));

    }
}
