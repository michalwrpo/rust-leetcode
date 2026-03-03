use std::collections::BinaryHeap;
use std::cmp;
use std::cmp::Ordering;

struct Solution;

#[derive(Eq, PartialEq)]
struct Node {
    r: usize,
    c: usize,
    cost: i32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist: Vec<_> = (0..n).map(|_| vec![i32::MAX;n]).collect();
        let mut heap = BinaryHeap::new();

        dist[0][0] = grid[0][0];

        let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        heap.push(Node { r: 0, c: 0, cost: grid[0][0] });

        while let Some(Node {r, c, cost}) = heap.pop() {
            if (r, c) == (n - 1, n - 1) { break; }

            if cost > dist[r][c] { continue; }

            for (x, y) in dirs {
                let nr = r as i32 + y;
                let nc = c as i32 + x;

                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 { continue; }

                let (nr, nc) = (nr as usize, nc as usize);

                let d = cmp::max(cost, grid[nr][nc]);

                if d >= dist[nr][nc] { continue; }
                
                heap.push(Node { r: nr, c: nc, cost: d });
                dist[nr][nc] = d;
            }
        }

        dist[n-1][n-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid2x2() {
        let grid = vec![vec![0,2], vec![1,3]];

        let ans = Solution::swim_in_water(grid);

        assert_eq!(ans, 3);
    }

    #[test]
    fn grid5x5() {
        let grid = vec![vec![0,1,2,3,4],vec![24,23,22,21,5],vec![12,13,14,15,16],vec![11,17,18,19,20],vec![10,9,8,7,6]];

        let ans = Solution::swim_in_water(grid);

        assert_eq!(ans, 16);
    }
}