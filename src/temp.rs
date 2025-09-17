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

use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n % 2 == 0 {
        return vec![];
    }

    use std::collections::HashMap;

    fn dfs(
        n: i32,
        memo: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(vec) = memo.get(&n) {
            return vec.clone();
        }

        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        let mut result = vec![];
        let mut node = TreeNode::new(0);

        for i in (1..n - 1).step_by(2) {
            let left_parts = dfs(i, memo);
            let right_parts = dfs(n - i - 1, memo);

            for left_part in &left_parts {
                for right_part in &right_parts {
                    result.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left_part.clone(),
                        right: right_part.clone(),
                    }))));
                }
            }
        }

        memo.insert(n, result.clone());
        result
    }

    let mut memo = HashMap::new();

    dfs(n, &mut memo)
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut squares = vec![HashSet::new(); 9];

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '.' {
                continue;
            }
            let digit = board[i][j];
            rows[i].insert(digit);
            cols[j].insert(digit);

            let square = i / 3 * 3 + j / 3;
            squares[square].insert(digit);
        }
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        rows: &mut Vec<HashSet<char>>,
        cols: &mut Vec<HashSet<char>>,
        squares: &mut Vec<HashSet<char>>,
    ) -> bool {
        if i == 9 {
            return true;
        }

        if j == 9 {
            return backtrack(board, i + 1, 0, rows, cols, squares);
        }

        if board[i][j] != '.' {
            return backtrack(board, i, j + 1, rows, cols, squares);
        }

        let square = i / 3 * 3 + j / 3;

        for ch in '1'..'9' {
            if rows[i].contains(&ch) || cols[j].contains(&ch) || squares[square].contains(&ch) {
                continue;
            }

            rows[i].insert(ch);
            cols[j].insert(ch);
            squares[square].insert(ch);
            board[i][j] = ch;

            if backtrack(board, i, j + 1, rows, cols, squares) {
                return true;
            }

            rows[i].remove(&ch);
            cols[j].remove(&ch);
            squares[square].remove(&ch);
            board[i][j] = '.';
        }

        false
    }

    backtrack(board, 0, 0, &mut rows, &mut cols, &mut squares);
}

const MARGIN: i32 = 300;

struct RobotStatistics {
    requests: Vec<(i32, i32)>,
}

impl RobotStatistics {
    fn on_event(&mut self, now: i32, user_id: i32) {
        if self.requests.len() == 0 {
            return;
        }
        let first = self.requests[0].0;
        if now - first > MARGIN {
            self.requests = self.requests[1..].iter().clone().collect();
        }
        self.requests.push((now, user_id));
    }

    fn get_robot_count(&self, now: i32) -> i32 {
        let n = self.requests.len();

        let last_request_time = now;

        let mut start = 0;
        let mut end = self.requests.len();

        while start < end {
            let mid = start + (end - start) / 2;

            if last_request_time - self.requests[mid].0 <= MARGIN {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        let mut map = std::collections::HashMap::new();

        for i in end..n {
            let id = self.requests[i].1;
            *map.entry(id).or_insert(0) += 1;
        }

        map.into_iter().filter(|&(_, count)| count >= 1000).count() as i32
    }
}

pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = 0;

    let mut dp = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = 1 + vec![dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]]
                        .into_iter()
                        .min()
                        .unwrap();
                }
                result += dp[i][j];
            }
        }
    }

    result
}

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = 0;

    let mut is_found = false;

    let mut left = 0;
    for right in 0..n {
        if nums[right] == 0 {
            if is_found {
                while nums[left] != 0 && left < 0 {
                    left += 1;
                }
            } else {
                is_found = true;
            }
        }

        result = result.max(right - left - 1 - if is_found { 1 } else { 0 });
    }

    result
}

use std::cmp::Ordering;
#[derive(Debug)]
struct Class {
    pass: i32,
    total: i32,
    gain: f64,
}

impl Class {
    pub fn new(pass: i32, total: i32) -> Self {
        let gain = ((pass + 1) as f64 / (total + 1) as f64) - (pass as f64 / total as f64);
        Class { pass, total, gain }
    }

    pub fn ratio(&self) -> f64 {
        self.pass as f64 / self.total as f64
    }

    pub fn update(&mut self) {
        self.pass += 1;
        self.total += 1;
        self.gain = ((self.pass + 1) as f64 / (self.total + 1) as f64)
            - self.pass as f64 / self.total as f64;
    }
}

impl Eq for Class {}

impl PartialEq for Class {}
impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gain.partial_cmp(&other.gain).unwrap()
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gain.partial_cmp(&other.gain)
    }
}

pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    use std::collections::BinaryHeap;
    let n = classes.len() as f64;
    let mut min_heap = BinaryHeap::new();

    for class in classes {
        let class = Class::new(class[0], class[1]);
        min_heap.push(class);
    }

    for _ in 0..extra_students {
        let mut class = min_heap.pop().unwrap();
        class.update();
        min_heap.push(class);
    }

    let mut result = 0.0;
    while let Some(class) = min_heap.pop() {
        result += class.pass as f64 / class.total as f64;
    }
    result / n
}

pub fn sum_zero(n: i32) -> Vec<i32> {
    let mut result = vec![];

    for i in 1..=n / 2 {
        result.push(i);
        result.push(-i);
    }

    if n % 2 == 1 {
        result.push(0);
    }

    result
}
