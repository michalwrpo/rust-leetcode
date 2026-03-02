fn main() {
    let grid = vec![
        vec![0,0,1],
        vec![1,1,0],
        vec![1,0,0],
    ];
    let result = Solution::min_swaps(grid);

    println!("{result}");
}

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let size = grid.len();
        let mut last_ones = Vec::new();

        for row in grid {
            let mut last_one = 0;
            for i in 0..size {
                if row[i] != 0 {
                    last_one = i;
                }
            }
            last_ones.push(last_one);
        }
        
        let mut swaps = 0;
        
        for i in 0..size {
            if last_ones[i] > i {
                let mut index = size;

                for j in i + 1..size {
                    if last_ones[j] <= i {
                        index = j;
                        break;
                    }
                }

                if index == size {
                    return -1;
                }

                for j in (i..index).rev() {
                    last_ones.swap(j, j + 1);
                    swaps += 1;
                }
            }
        }

        swaps
    }
}