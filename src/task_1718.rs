/*Given an integer n, find a sequence that satisfies all of the following:

The integer 1 occurs once in the sequence.
Each integer between 2 and n occurs twice in the sequence.
For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.

The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their indices, |j - i|.

Return the lexicographically largest sequence. It is guaranteed that under the given constraints, there is always a solution.

A sequence a is lexicographically larger than a sequence b (of the same length) if in the first position where a and b differ, sequence a has a number greater than the corresponding number in b. For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they differ is at the third number, and 9 is greater than 5.



Example 1:

Input: n = 3
Output: [3,1,2,3,2]
Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.

Example 2:

Input: n = 5
Output: [5,3,1,4,3,5,2,4,2]



Constraints:

1 <= n <= 20

*/
use std::cmp::max;

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let len = n as usize * 2 - 1;
    fn backtrack(sequence: &mut Vec<i32>, idx: usize, n: usize, visited: &mut Vec<bool>, max_sequence: &mut Vec<i32>) -> bool {
        if idx == n {
            *max_sequence = sequence.clone();
            return true;
        }

        if sequence[idx] == -1 {
            return backtrack(sequence, idx +1, n, visited, max_sequence);
        }

        for num in (1..=n).rev() {
            if visited[num] {
                continue;
            }

            if num == 1 {
                sequence[idx] = 1;
                visited[num] = true;
                if backtrack(sequence, idx + 1, n, visited, max_sequence) {
                    return true;
                }
                visited[num] = false;
                sequence[idx] = -1;
            } else {
                sequence[idx] = num as i32;
                visited[num] = true;

                if idx + num < sequence.len() && sequence[idx+num] == -1 {
                    sequence[idx+num] = num as i32;

                    if backtrack(sequence, idx + 1, n, visited, max_sequence) {
                        return true;
                    }

                    sequence[idx+num] = num as i32;
                }

                visited[num] = false;
                sequence[idx] = -1;
            }
        }

        false
    }

    let mut sequence = vec![-1; len];
    let mut max_sequence = vec![-1; len];
    let mut visited = vec![false; n as usize + 1];
    backtrack(&mut sequence, 0, n as usize, &mut visited, &mut max_sequence);

    max_sequence.clone()
}