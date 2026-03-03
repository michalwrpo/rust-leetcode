struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut result = vec![0; spells.len()];
        let mut sorted_potions = potions.clone();
        sorted_potions.sort();

        let pot_number = sorted_potions.len() as i32;
        
        for (i, spell) in spells.into_iter().enumerate() {
            let mut l = -1;
            let mut r = pot_number;
            while r - l > 1 {
                let mid = l + (r - l) / 2;
                if (spell as i64) * (sorted_potions[mid as usize] as i64) < success { 
                    l = mid;
                } else {
                    r = mid;
                }
            }

            result[i] += pot_number - l - 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let spells = vec![5,1,3];
        let potions = vec![1,2,3,4,5];
        let success = 7;

        let ans = Solution::successful_pairs(spells, potions, success);

        assert_eq!(ans, vec![4,0,3]);
    }

    #[test]
    fn test2() {
        let spells = vec![3,1,2];
        let potions = vec![8,5,8];
        let success = 16;

        let ans = Solution::successful_pairs(spells, potions, success);

        assert_eq!(ans, vec![2,0,2]);
    }
}