fn main() {
    let h = vec![1,8,6,2,5,4,8,3,7];
    let ans = Solution::max_area(h);

    println!("{ans}");
}

use std::cmp;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let current_height = cmp::min(height[left], height[right]);
            let width = right - left;
            
            let area = width as i32 * current_height;

            max_area = cmp::max(area, max_area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}