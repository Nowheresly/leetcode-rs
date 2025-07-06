use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    map2: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for i in 0..nums2.len() {
            *map.entry(nums2[i]).or_insert(0) += 1;
        }
        Self {nums1, nums2, map2: map}
    }

    fn add(&mut self, index: i32, val: i32) {
        let prev_val = &self.nums2.iter().nth(index as usize).unwrap();
        *self.map2.entry(**prev_val).or_insert(0) -= 1;
        self.nums2[index as usize] += val;
        *self.map2.entry(self.nums2[index as usize]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ret = 0;
        for &num1 in &self.nums1 {
            let target = tot - num1;
            if let Some(&count) = self.map2.get(&target) {
                ret += count;
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 1, 2, 2, 2, 3];
        let nums2 = vec![1, 4, 5, 2, 5, 4];
        let mut obj = FindSumPairs::new(nums1, nums2);
        assert_eq!(8, obj.count(7));
        obj.add(3, 2);
        assert_eq!(2, obj.count(8));
        assert_eq!(1, obj.count(4));
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(11, obj.count(7));
    }
}
