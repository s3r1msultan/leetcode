/*You may recall that an array arr is a mountain array if and only if:

arr.length >= 3
There exists some index i (0-indexed) with 0 < i < arr.length - 1 such that:
arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

Given an integer array arr, return the length of the longest subarray, which is a mountain. Return 0 if there is no mountain subarray.



Example 1:

Input: arr = [2,1,4,7,3,2,5]
Output: 5
Explanation: The largest mountain is [1,4,7,3,2] which has length 5.

Example 2:

Input: arr = [2,2,2]
Output: 0
Explanation: There is no mountain.



Constraints:

1 <= arr.length <= 104
0 <= arr[i] <= 104



Follow up:

Can you solve it using only one pass?
Can you solve it in O(1) space?

*/

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    if arr.len() < 3 {
        return 0;
    }

    let mut left = 0;

    let mut is_increasing = false;
    let mut is_decreasing = false;

    let mut result = 0;

    for right in 1..arr.len() {
        if arr[right - 1] == arr[right] {
            left = right;
            continue;
        }

        if arr[right - 1] < arr[right] {
            if is_decreasing {
                is_decreasing = false;
                left = right;
            }
            if !is_increasing {
                is_increasing = true;
            }
        } else {
            if is_increasing && !is_decreasing {
                is_decreasing = true;
                is_increasing = false;
            }
        }

        result = result.max(right - left + 1);
    }

    result as i32
}