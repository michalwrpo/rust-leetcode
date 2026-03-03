use std::collections::HashMap;
use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut drain = vec![-1;rains.len()];

        let mut full = HashMap::new();
        let mut free = BTreeSet::new();

        for (i, l) in rains.into_iter().enumerate() {
            if l == 0 {
                drain[i] = 1;
                free.insert(i);
                continue;
            }

            if let Some(prev) = full.insert(l, i) {
                match free.range(prev..).next() {
                    Some(&day) => {
                        drain[day] = l;
                        free.remove(&day);
                    }
                    None => return vec![]
                }
            }
        }

        drain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rains = vec![1,2,3,4];

        let ans = Solution::avoid_flood(rains);

        assert_eq!(ans, vec![-1,-1,-1,-1]);
    }

    #[test]
    fn test2() {
        let rains = vec![1,2,0,0,2,1];

        let ans = Solution::avoid_flood(rains);

        assert_eq!(ans, vec![-1,-1,2,1,-1,-1]);
    }

    #[test]
    fn test3() {
        let rains = vec![1,2,0,1,2];

        let ans = Solution::avoid_flood(rains);

        assert_eq!(ans, vec![]);
    }

    #[test]
    fn test4() {
        let rains = vec![0,1,1];

        let ans = Solution::avoid_flood(rains);

        assert_eq!(ans, vec![]);
    }

    #[test]
    fn test5() {
        let rains = vec![0,0,1,1];

        let ans = Solution::avoid_flood(rains);

        assert_eq!(ans, vec![]);
    }
}