use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let ch: Vec<char> = s.chars().collect();
        let n = ch.len();
        let mut ans: i32 = 0;
        let mut i = 0;
        while i < n {
            let start = i;
            i += 1;
            while i < n && ch[i] == ch[i - 1] {
                i += 1;
            }
            ans = ans.max(i as i32 - start as i32);
        }
        ans = ans.max(two_chars(&ch, n, 'a', 'b'));
        ans = ans.max(two_chars(&ch, n, 'a', 'c'));
        ans = ans.max(two_chars(&ch, n, 'b', 'c'));

        let mut pos = HashMap::new();
        pos.insert(key(0, 0), -1);

        let mut ca = 0;
        let mut cb = 0;
        let mut cc= 0;
        for i in 0..n {
            let c = ch[i];
            if c == 'a' {
                ca += 1;
            } else if c == 'b' {
                cb += 1;
            } else {
                cc += 1;
            }

            let d1 = ca - cb;
            let d2 = cb - cc;
            let k = key(d1, d2);
            if pos.contains_key(&k) {
                ans = ans.max(i as i32 - pos[&k]);
            } else {
                pos.insert(k, i as i32);
            }
        }
        ans
    }
}

fn key(d1: i32, d2: i32) -> i64 {
    ((d1 as i64) << 32) ^ (d2 as i64 & 0xffffffffi64)
}

fn two_chars(ch: &Vec<char>, n: usize, x: char, y: char) -> i32 {
    let mut best = 0;
    let mut first = vec![i32::MAX; 2 * n + 1];
    let mut touched = vec![0; 2 * n + 1];
    let mut touched_size = 0;
    let mut i = 0;
    while i < n {
        let seg_start = i as i32;

        first[n] = seg_start - 1;
        touched[touched_size] = n;
        touched_size += 1;

        let mut d:i32 = 0;
        while i < n {
            let c = ch[i];
            if c != x && c != y {
                break;
            }
            d += if c == x { 1 } else {-1 };
            let idx = d + n as i32;

            if first[idx as usize] != i32::MAX {
                best = best.max(i as i32 - first[idx as usize]);
            } else {
                first[idx as usize] = i as i32;
                touched[touched_size] = idx as usize;
                touched_size += 1;
            }
            i += 1;
        }
        for k in 0..touched_size {
            first[touched[k]] = i32::MAX;
        }
        touched_size = 0;
        i += 1;
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::longest_balanced(String::from("abbac")));
        assert_eq!(3, Solution::longest_balanced(String::from("aabcc")));
    }
}
