use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 280. Wiggle Sort
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        // 0, 2, 1, 3
        let N = nums.len();
        let mut counters: HashMap<i32, i32> = HashMap::new();
        let mut small = nums[0];
        let mut big = nums[0];

        for i in 0..N {
            let count = counters.entry(nums[i]).or_insert(0);
            *count += 1;
            small = small.min(nums[i]);
            big = big.max(nums[i]);
        }

        let mut left = 0;
        let mut right = 1;
        while small <= big {
            match counters.get_mut(&small) {
                Some(cnt) => {
                    if left <= right + 1 && left < N {
                        nums[left] = small;
                        left += 2;
                    } else {
                        nums[right] = small;
                        right += 2;
                    }
                    *cnt -= 1;
                    if *cnt == 0 {
                        small += 1;
                    }
                }
                None => {
                    small += 1;
                    continue;
                }
            }
        }
    }
}
