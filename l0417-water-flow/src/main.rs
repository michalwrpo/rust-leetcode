struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut oceans = Vec::with_capacity(m);
        let mut result = Vec::new();

        for _ in 0..m {
            let mut row = Vec::with_capacity(n);
            for _ in 0..n {
                row.push(0);
            }
            oceans.push(row);
        }

        for i in 0..m {
            check_square(&mut oceans, &heights, i, 0);
            check_square(&mut oceans, &heights, i, n-1);
        }

        for j in 0..n {
            check_square(&mut oceans, &heights, 0, j);
            check_square(&mut oceans, &heights, m-1, j);

        }

        for i in 0..m {
            for j in 0..n {
                if oceans[i][j] == 3 {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }

        result
    }
}

fn check_square(oceans: &mut Vec<Vec<i8>>, heights: &Vec<Vec<i32>>, i: usize, j: usize) {
    if i == 0 || j == 0 {
        oceans[i][j] |= 1;
    }

    if i == oceans.len() - 1 || j == oceans[0].len() - 1 {
        oceans[i][j] |= 2;
    }

    if i > 0 {
        let i1 = i-1;
        let j1 = j;
    
        if heights[i1][j1] >= heights[i][j] && (oceans[i1][j1] | oceans[i][j] > oceans[i1][j1]) {
            oceans[i1][j1] |= oceans[i][j];
            check_square(oceans, heights, i1, j1);
        }
    }

    if j > 0 {
        let i1 = i;
        let j1 = j-1;
    
        if heights[i1][j1] >= heights[i][j] && (oceans[i1][j1] | oceans[i][j] > oceans[i1][j1]) {
            oceans[i1][j1] |= oceans[i][j];
            check_square(oceans, heights, i1, j1);
        }
    }

    if i < oceans.len() - 1 {
        let i1 = i+1;
        let j1 = j;
    
        if heights[i1][j1] >= heights[i][j] && (oceans[i1][j1] | oceans[i][j] > oceans[i1][j1]) {
            oceans[i1][j1] |= oceans[i][j];
            check_square(oceans, heights, i1, j1);
        }
    }

    if j < oceans[0].len() - 1 {
        let i1 = i;
        let j1 = j+1;
    
        if heights[i1][j1] >= heights[i][j] && (oceans[i1][j1] | oceans[i][j] > oceans[i1][j1]) {
            oceans[i1][j1] |= oceans[i][j];
            check_square(oceans, heights, i1, j1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid5x5() {
        let heights = vec![
            vec![1,2,2,3,5],
            vec![3,2,3,4,4],
            vec![2,4,5,3,1],
            vec![6,7,1,4,5],
            vec![5,1,1,2,4]
        ];

        let ans = Solution::pacific_atlantic(heights);

        assert_eq!(ans, vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]])
    }

    #[test]
    fn test_grid3x6() {
        let heights = vec![
            vec![3,3,3,3,3,3],
            vec![3,0,3,3,0,3],
            vec![3,3,3,3,3,3]
        ];

        let ans = Solution::pacific_atlantic(heights);

        assert_eq!(ans, vec![vec![0,0],vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5],vec![1,0],vec![1,2],vec![1,3],vec![1,5],vec![2,0],vec![2,1],vec![2,2],vec![2,3],vec![2,4],vec![2,5]])

    }
}