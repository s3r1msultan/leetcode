/*You have n jobs and m workers. You are given three arrays: difficulty, profit, and worker where:

difficulty[i] and profit[i] are the difficulty and the profit of the ith job, and
worker[j] is the ability of jth worker (i.e., the jth worker can only complete a job with difficulty at most worker[j]).

Every worker can be assigned at most one job, but one job can be completed multiple times.

For example, if three workers attempt the same job that pays $1, then the total profit will be $3. If a worker cannot complete any job, their profit is $0.

Return the maximum profit we can achieve after assigning the workers to the jobs.



Example 1:

Input: difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
Output: 100
Explanation: Workers are assigned jobs of difficulty [4,4,6,6] and they get a profit of [20,20,30,30] separately.

Example 2:

Input: difficulty = [85,47,57], profit = [24,66,99], worker = [40,25,25]
Output: 0



Constraints:

n == difficulty.length
n == profit.length
m == worker.length
1 <= n, m <= 104
1 <= difficulty[i], profit[i], worker[i] <= 105

*/


fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
    let len = jobs.len();
    jobs.sort_unstable_by(|a, b| if a.0 == b.0 { b.1.cmp(&a.1) } else { a.0.cmp(&b.0) });
    for capability in worker {
        max_profit += linear_search(capability, &jobs);
    }

    // fn binary_search(low: usize, high: usize, worker_capability: i32, arr: &Vec<(i32, i32)>) -> i32 {
    //     let mid = (low + high) / 2;
    //
    //     if low >= high {
    //         return *arr[mid].0;
    //     }
    //
    //     return
    //         if arr[mid].0 > worker_capability {
    //             binary_search(low, mid - 1, worker_capability, &arr)
    //         } else {
    //             binary_search(mid + 1, high, worker_capability, &arr)
    //         };
    // }

    fn linear_search(capability: i32, jobs: &Vec<(i32, i32)>) -> i32 {
        let mut max_profit = 0;
        for &(difficulty, profit) in jobs.iter() {
            if difficulty <= capability {
                max_profit = max_profit.max(profit);
            } else { break; }
        }
        max_profit
    }

    max_profit
}

#[cfg(test)]
#[test]
fn test_max_profit_assignment() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    let result = 100;
    assert_eq!(max_profit_assignment(difficulty, profit, worker), result);

    let difficulty = vec![85, 47, 57];
    let profit = vec![24, 66, 99];
    let worker = vec![40, 25, 25];
    let result = 0;
    assert_eq!(max_profit_assignment(difficulty, profit, worker), result);
}