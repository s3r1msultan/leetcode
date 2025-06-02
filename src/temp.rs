/*Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.



Example 1:

Input: nums = [1,5,11,5]
Output: true
Explanation: The array can be partitioned as [1, 5, 5] and [11].

Example 2:

Input: nums = [1,2,3,5]
Output: false
Explanation: The array cannot be partitioned into equal sum subsets.



Constraints:

1 <= nums.length <= 200
1 <= nums[i] <= 100

*/

pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let n = nums.len();
    let half_sum = sum as usize / 2;
    let mut dp = vec![vec![0; half_sum + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..half_sum + 1 {
            if nums[i - 1] > j as i32 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(nums[i - 1] + dp[i - 1][j - nums[i - 1] as usize]);
            }
        }
    }
    dp[n][half_sum] == half_sum as i32
}


pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let n = move_time.len();
    let m = move_time[0].len();
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    let mut grid = vec![vec![i32::MAX; m]; n];
    let mut min_heap = std::collections::BinaryHeap::new();
    grid[0][0] = 0;
    min_heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((time, i, j))) = min_heap.pop() {
        if i == n - 1 && j == m - 1 {
            return time;
        }

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                break;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            let new_time = if time >= move_time[ni][nj] {
                time
            } else {
                move_time[ni][nj]
            };

            if new_time + 1 < grid[ni][nj] {
                grid[ni][nj] = new_time + 1;
                min_heap.push(Reverse((grid[ni][nj], ni, nj)));
            }
        }
    }

    unreachable!()
}

pub fn three_consecutive_odds(nums: Vec<i32>) -> bool {
    let mut count = 0;
    
    for num in nums {
        if num % 2 == 1 {
            count += 1;
        } else {
            count = 0;
        }

        if count == 3 {
            return true;
        }
    }

    false
}