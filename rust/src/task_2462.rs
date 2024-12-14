/*You are given a 0-indexed integer array costs where costs[i] is the cost of hiring the ith worker.

You are also given two integers k and candidates. We want to hire exactly k workers according to the following rules:

You will run k sessions and hire exactly one worker in each session.
In each hiring session, choose the worker with the lowest cost from either the first candidates workers or the last candidates workers. Break the tie by the smallest index.
For example, if costs = [3,2,7,7,1,2] and candidates = 2, then in the first hiring session, we will choose the 4th worker because they have the lowest cost [3,2,7,7,1,2].
In the second hiring session, we will choose 1st worker because they have the same lowest cost as 4th worker but they have the smallest index [3,2,7,7,2]. Please note that the indexing may be changed in the process.
If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them. Break the tie by the smallest index.
A worker can only be chosen once.

Return the total cost to hire exactly k workers.



Example 1:

Input: costs = [17,12,10,2,7,2,11,20,8], k = 3, candidates = 4
Output: 11
Explanation: We hire 3 workers in total. The total cost is initially 0.
- In the first hiring round we choose the worker from [17,12,10,2,7,2,11,20,8]. The lowest cost is 2, and we break the tie by the smallest index, which is 3. The total cost = 0 + 2 = 2.
- In the second hiring round we choose the worker from [17,12,10,7,2,11,20,8]. The lowest cost is 2 (index 4). The total cost = 2 + 2 = 4.
- In the third hiring round we choose the worker from [17,12,10,7,11,20,8]. The lowest cost is 7 (index 3). The total cost = 4 + 7 = 11. Notice that the worker with index 3 was common in the first and last four workers.
The total hiring cost is 11.

Example 2:

Input: costs = [1,2,4,1], k = 3, candidates = 3
Output: 4
Explanation: We hire 3 workers in total. The total cost is initially 0.
- In the first hiring round we choose the worker from [1,2,4,1]. The lowest cost is 1, and we break the tie by the smallest index, which is 0. The total cost = 0 + 1 = 1. Notice that workers with index 1 and 2 are common in the first and last 3 workers.
- In the second hiring round we choose the worker from [2,4,1]. The lowest cost is 1 (index 2). The total cost = 1 + 1 = 2.
- In the third hiring round there are less than three candidates. We choose the worker from the remaining workers [2,4]. The lowest cost is 2 (index 0). The total cost = 2 + 2 = 4.
The total hiring cost is 4.



Constraints:

1 <= costs.length <= 105
1 <= costs[i] <= 105
1 <= k, candidates <= costs.length

*/

pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let n = costs.len();
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();
    let mut left_ptr = candidates as usize - 1;
    let mut right_ptr = (n - candidates as usize).max(left_ptr + 1);
    let mut max_total = 0i64;

    for i in 0..candidates as usize {
        left_heap.push(Reverse((costs[i], i)));
    }

    for i in right_ptr..n {
        right_heap.push(Reverse((costs[i], i)));
    }

    for _ in 0..k {
        max_total += match (left_heap.peek(), right_heap.peek()) {
            (Some(&Reverse((left, i))), Some(&Reverse((right, j)))) => {
                if left > right {
                    right_heap.pop();
                    if left_ptr < right_ptr - 1 {
                        right_ptr -= 1;
                        right_heap.push(Reverse((costs[right_ptr], right_ptr)));
                    }
                    right as i64
                } else if left < right {
                    left_heap.pop();
                    if left_ptr + 1 < right_ptr {
                        left_ptr += 1;
                        left_heap.push(Reverse((costs[left_ptr], left_ptr)));
                    }
                    left as i64
                } else {
                    if i < j {
                        left_heap.pop();
                        if left_ptr + 1 < right_ptr {
                            left_ptr += 1;
                            left_heap.push(Reverse((costs[left_ptr], left_ptr)));
                        }
                        left as i64
                    } else {
                        right_heap.pop();
                        if left_ptr < right_ptr - 1 {
                            right_ptr -= 1;
                            right_heap.push(Reverse((costs[right_ptr], right_ptr)));
                        }
                        right as i64
                    }
                }
            }

            (_, Some(&Reverse((right, i)))) => {
                right_heap.pop();
                right as i64
            }
            (Some(&Reverse((left, i))), _) => {
                left_heap.pop();
                left as i64
            }

            (None, None) => break
        };
    }

    max_total
}