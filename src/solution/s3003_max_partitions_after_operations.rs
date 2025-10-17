
pub struct Solution {}

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == 26 {
            return 1;
        }
        let n = s.len();
        let mut left_mask = vec![0; n];
        let mut left_dup = vec![0; n];
        let mut left_parts = vec![0; n];

        let mut mask:i32 = 0;
        let mut dup = 0;
        let mut parts = 1;

        for i in 0..n {
            let bit = 1 << (s.chars().nth(i).unwrap() as usize - 'a' as usize);
            dup = dup | (mask & bit);
            mask = mask | bit;
            if mask.count_ones() > k as u32 {
                mask = bit;
                dup = 0;
                parts += 1;
            }
            left_mask[i] = mask;
            left_dup[i] = dup;
            left_parts[i] = parts;
        }

        let mut results = parts;
        mask = 0;
        dup = 0;
        parts = 0;

        for i in (0..n).rev() {
            let bit = 1 << (s.chars().nth(i).unwrap() as usize - 'a' as usize);
            dup = dup | (mask & bit);
            mask = mask | bit;
            let bit_count = mask.count_ones();
            if bit_count > k as u32 {
                mask = bit;
                dup = 0;
                parts += 1;
            }
            if bit_count == k as u32 {
                if bit & dup != 0 && bit & left_dup[i] != 0 && left_mask[i].count_ones() == k as u32 && left_mask[i] | mask != 0x3FFFFFF {
                    results = results.max(left_parts[i] + parts + 2);
                    //parts += 1;
                } else if dup != 0 {
                    results = results.max(left_parts[i] + parts + 1);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_partitions_after_operations(String::from("accca"), 2));
        assert_eq!(1, Solution::max_partitions_after_operations(String::from("aabaab"), 3));
        assert_eq!(4, Solution::max_partitions_after_operations(String::from("xxyz"), 1));
        assert_eq!(3, Solution::max_partitions_after_operations(String::from("abbaaca"), 2));

    }
}
