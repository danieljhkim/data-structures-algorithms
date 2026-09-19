pub struct Solution;

impl Solution {
    // 3904. Smallest Stable Index II
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut min_values = vec![0; n];
        min_values[n - 1] = nums[n - 1];
        let mut min_val = min_values[n - 1];

        for i in (0..n - 1).rev() {
            min_val = min_val.min(nums[i]);
            min_values[i] = min_val;
        }

        let mut max_val = 0;
        for i in 0..n {
            max_val = max_val.max(nums[i]);
            if max_val - min_values[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stabilizes_partway_through() {
        assert_eq!(Solution::first_stable_index(vec![3, 1, 2, 2, 2], 1), 2);
    }

    #[test]
    fn never_stabilizes() {
        assert_eq!(Solution::first_stable_index(vec![5, 4, 3, 2, 1], 0), -1);
    }
}
