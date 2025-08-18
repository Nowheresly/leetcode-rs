pub struct Solution {}

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        dfs(&cards, 0f64)
    }
}

fn dfs(cards: &Vec<i32>, sum: f64) -> bool {
    if cards.is_empty() {
        if (sum - 24f64).abs() < 0.001 {
            return true;
        }
        return false;
    }

    if cards.len() == 4 {
        for i in 0..cards.len() {
            for j in 0..cards.len() {
                if i == j {
                    continue;
                }
                let val1 = *cards.get(i).unwrap() as f64;
                let val2 = *cards.get(j).unwrap() as f64;
                let mut new_cards = cards.clone();
                if i > j {
                    new_cards.remove(i);
                    new_cards.remove(j);
                } else {
                    new_cards.remove(j);
                    new_cards.remove(i);
                }

                if dfs(&new_cards, val1 + val2) {
                    return true;
                }
                if dfs(&new_cards, val1 - val2) {
                    return true;
                }
                if dfs(&new_cards, val2 - val1) {
                    return true;
                }
                if dfs(&new_cards, val1 * val2) {
                    return true;
                }
                if val2 != 0f64 && dfs(&new_cards, val1 / val2) {
                    return true;
                }
                if val1 != 0f64 && dfs(&new_cards, val2 / val1) {
                    return true;
                }
            }
        }
        return false;
    }
    if cards.len() == 2 {
        let val1 = *cards.get(0).unwrap() as f64;
        let val2 = *cards.get(1).unwrap() as f64;

        if dfs(&vec![], sum * (val1 + val2)) {
            return true;
        }
        if dfs(&vec![], sum * (val1 - val2)) {
            return true;
        }
        if dfs(&vec![], sum * (val2 - val1)) {
            return true;
        }
        if dfs(&vec![], sum / (val1 + val2)) {
            return true;
        }
        if dfs(&vec![], sum / (val1 - val2)) {
            return true;
        }
        if dfs(&vec![], sum / (val2 - val1)) {
            return true;
        }
    }

    for i in 0..cards.len() {
        let val = *cards.get(i).unwrap() as f64;
        let mut new_cards = cards.clone();
        new_cards.remove(i);
        if dfs(&new_cards, sum + val) {
            return true;
        }
        if dfs(&new_cards, sum - val) {
            return true;
        }
        if dfs(&new_cards, val - sum) {
            return true;
        }
        if dfs(&new_cards, sum * val) {
            return true;
        }
        if val != 0f64 && dfs(&new_cards, sum / val) {
            return true;
        }
        if sum != 0f64 && dfs(&new_cards, val / sum) {
            return true;
        }
        if val != 0f64 && dfs(&new_cards, sum * val) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::judge_point24(vec![4, 1, 8, 7]));
        assert_eq!(false, Solution::judge_point24(vec![1, 2, 1, 2]));
        assert_eq!(true, Solution::judge_point24(vec![1, 3, 4, 6]));
    }
}
