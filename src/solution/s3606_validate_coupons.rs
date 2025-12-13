use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut activities = HashSet::new();
        activities.insert(String::from("electronics"));
        activities.insert(String::from("grocery"));
        activities.insert(String::from("pharmacy"));
        activities.insert(String::from("restaurant"));
        let n = code.len();
        let mut valids = vec![];
        for i in 0..n {
            let cd = &code[i];
            if is_active[i] == false {
                continue;    
            }
            if activities.contains(&business_line[i]) == false {
                continue;
            }
            if  cd.is_empty() {
                continue;
            }
            let mut ok = true;
            for c in cd.chars() {
                if c == '_'  ||  c.is_alphanumeric() {
                    continue;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok == false {
                continue;
            }
            valids.push((cd.clone(), business_line[i].clone()));
        }
        valids.sort_by(|a, b| {
            let biz = a.1.cmp(&b.1);
            if biz == std::cmp::Ordering::Equal {
                a.0.cmp(&b.0)
            } else {
                biz
            }
        });
        valids.into_iter().map(|x| x.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![String::from("PHARMA5"), String::from("SAVE20")], Solution::validate_coupons(vec![String::from("SAVE20"), String::from(""), String::from("PHARMA5"), String::from("SAVE@20")],
                                                                                                     vec![String::from("restaurant"), String::from("grocery"), String::from("pharmacy"), String::from("restaurant")],
                                                                                                     vec![true, true, true, true]));

    }
}
