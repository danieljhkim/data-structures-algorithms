pub struct Solution;

impl Solution {
    // 1762. Buildings With an Ocean View
    pub fn find_buildings(mut heights: Vec<i32>) -> Vec<i32> {
        let mut big = i32::MIN;
        let mut ans: Vec<i32> = Vec::new();

        while let Some(cur) = heights.pop() {
            if cur > big {
                ans.push(heights.len() as i32);
                big = cur;
            }
        }
        ans.reverse();
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_buildings(vec![4, 2, 3, 1]), vec![0, 2, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_buildings(vec![4, 3, 2, 1]), vec![0, 1, 2, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_buildings(vec![1, 3, 2, 4]), vec![3]);
    }
}
