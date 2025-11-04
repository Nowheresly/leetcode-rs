pub struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n - k as usize + 1];

        for i in 0.. (n-k as usize +1) {
            let mut freq = vec![0; 51];

            for j in i.. (i + k as usize) {
                freq[nums[j] as usize] += 1;
            }
            let mut pq:Vec<(i32,i32)> = vec![];
            for v in 0..51 {
                if freq[v] > 0 {
                    pq.push((v as i32, freq[v]));
                }
            }
            pq.sort_by(|a, b| {
                if b.1.cmp(&a.1) == std::cmp::Ordering::Equal {
                    b.0.cmp(&a.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });
            let mut sum = 0;
            let mut tmp = x;
            while tmp > 0 && pq.len() > 0 {
                let (val, count) = pq.remove(0);
                sum += val * count;
                tmp -= 1;
            }
            ans[i] = sum;

        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![6,10,12], Solution::find_x_sum(vec![1,1,2,2,3,4,2,3], 6, 2));
        assert_eq!(vec![11,15,15,15,12], Solution::find_x_sum(vec![3,8,7,8,7,5], 2, 2));
    }
}