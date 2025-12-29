use std::collections::HashMap;

pub struct Solution {

}

pub struct Data {
    memo: HashMap<String, bool>
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut data = Data { memo: HashMap::new() };

        dfs(&bottom, &allowed, &mut data, 0, String::from(""))
    }
}

fn dfs(bottom: &String, allowed: &Vec<String>, data: &mut Data, index: i32, next: String) -> bool {
    if bottom.len() == 1 {
        return true;
    }
    if index == 0 && data.memo.contains_key(bottom) {
        return data.memo.get(bottom).unwrap().clone();
    }
    if index == bottom.len() as i32 - 1 {
        let ret = dfs(&next, allowed, data,0, String::from(""));
        data.memo.insert(bottom.clone(), ret.clone());
        return ret;
    }
    let item = String::from(&bottom[index as usize..index as usize+2]);
    for all in allowed.iter() {
        if all.starts_with(&item) {
            let mut next = next.clone();
            next.push(all.as_bytes()[all.len() - 1] as char);
            let ret = dfs(bottom, allowed, data, index + 1, next);
            if ret {
                return ret;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::pyramid_transition(String::from("BCD"), vec![String::from("BCC"), String::from("CDE"), String::from("CEA"), String::from("FFF")]));
    }
}
