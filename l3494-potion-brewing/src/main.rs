use std::cmp;

struct Solution;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let m = mana.len();

        let mut time = vec![vec![0i64; n]; m];
        
        for (i, potion) in mana.into_iter().enumerate() {
            let mut delay = vec![0;n-1];

            for (j, wizard) in skill.iter().enumerate() {
                if i > 0 && j > 0{
                    time[i][j] = cmp::max(time[i-1][j], time[i][j-1]);
                } else if i > 0 {
                    time[i][j] = time[i-1][j];
                } else if j > 0 {
                    time[i][j] = time[i][j-1];
                } 

                time[i][j] += potion as i64 * *wizard as i64;
                if j < n - 1 && i > 0 {
                    delay[j] = cmp::max(time[i-1][j+1] - time[i][j], 0);
                }
            }

            let total_delay: i64 = delay.iter().sum();

            for (j, wizard) in skill.iter().enumerate() {
                if j == 0 {
                    time[i][j] += total_delay;
                } else {
                    time[i][j] = time[i][j-1] + potion as i64 * *wizard as i64;
                }
            }
        }


        *time.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let skill = vec![1,5,2,4];
        let mana = vec![5,1,4,2];

        let ans = Solution::min_time(skill, mana);

        assert_eq!(ans, 110);
    }

    #[test]
    fn test2() {
        let skill = vec![1,1,1];
        let mana = vec![1,1,1];

        let ans = Solution::min_time(skill, mana);

        assert_eq!(ans, 5);
    }

    #[test]
    fn test3() {
        let skill = vec![1,2,3,4];
        let mana = vec![1,2];

        let ans = Solution::min_time(skill, mana);

        assert_eq!(ans, 21);
    }
}